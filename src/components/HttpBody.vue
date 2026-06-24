<style scoped lang="scss">
.body {
  display: flex;
  flex-flow: column nowrap;
  overflow: auto;
  gap: var(--gap-md);
  padding: var(--gap-md);

  .body-controls {
    display: flex;
    flex-flow: row-reverse wrap;
    justify-content: space-between;

    .body-view-controls {
      display: grid;
      grid-template: "label label" "raw other";
      gap: 3px var(--gap-sm);

      strong {
        display: block;
        grid-area: label;
      }

      .body-view-raw {
        grid-area: raw;
      }

      .body-view-other {
        grid-area: other;
      }
    }

    .body-actions {
      display: flex;
      flex-flow: column;
      gap: 3px;

      strong span {
        text-transform: capitalize;
      }

      .body-action-list {
        display: flex;
        flex-flow: row wrap;
        gap: var(--gap-sm);
      }
    }
  }

  .html-preview {
    width: 100%;
    min-height: 600px;
    background-color: white;
  }

  pre,
  .json {
    background-color: #161616;
    border-radius: var(--radius-md);
    padding: var(--gap-md);
    overflow: auto;
    min-height: 220px;

    &::-webkit-scrollbar {
      height: 6px;
    }
  }

  .hex {
    height: 600px;
  }
}
</style>

<template>
  <div class="body">
    <div class="body-controls">
      <Button
          style="float: right"
          @click="copyToClipboard(textData, 'copyButton' + index)"
      >
        <ClipboardCopyIcon />
        <span :ref="'copyButton' + index">Copy</span>
      </Button>
      <div class="just-flex">
        <div class="body-view-controls">
          <strong>Preview as</strong>
          <Button
              class="body-view-raw"
              :color="show.bodyType === 'raw' ? 'secondary' : ''"
              @click="viewRaw"
          >
            Raw
          </Button>
          <DropdownButton
              class="body-view-other"
              :color="show.bodyType !== 'raw' ? 'secondary' : ''"
              :actions="bodyViewActions"
          />
        </div>
        <span v-if="contentActions != null">|</span>
        <div class="body-actions" v-if="contentActions != null">
          <strong><span>{{formattedPreview}}</span> actions</strong>
          <div class="body-action-list">
            <Button
                v-for="[name, func] in Object.entries(contentActions)"
                @click="func"
            >
              {{name}}
            </Button>
          </div>
        </div>
      </div>
    </div>

    <div v-if="show.bodyType === 'preview-html'">
      <iframe
          class="html-preview"
          :srcdoc="textData"
          frameborder="0"
      ></iframe>
    </div>

    <div v-else-if="show.bodyType === 'preview-image'">
      <img
          style="margin: 10px; width: unset; height: unset"
          :src="imageDataUrl"
          alt="COULD NOT LOAD IMAGE"
      />
    </div>

    <div v-else-if="show.bodyType === 'preview-json'">
      <div class="json"> <JsonNode ref="jsonNode" :data="textData" /> </div>
    </div>
    <div v-else-if="show.bodyType === 'preview-hex'">
      <div class="hex"> <HexViewer :bytes="byteData" /> </div>
    </div>

    <div v-else-if="resHighlightContent.length < 70000 && show.bodyType !== 'preview-text'">
      <AsyncHighlight :code="resHighlightContent" :language="highlightLang" />
    </div>

    <div v-else>
      <div class="error" v-if="resHighlightContent.length > 70000">REQUESTER-WARN: Body too long ({{resHighlightContent.length}}B/70000B)</div>
      <pre>{{ textData }}</pre>
    </div>
  </div>
</template>

<script>
import JsonNode from "./JsonNode.vue";
import {Button, Chips, ClipboardCopyIcon} from "omorphia";
import {copyToClipboard} from "../utils/simple.js";
import {findContentTypeInHeaders} from "../utils/http.js";
import AsyncHighlight from "./AsyncHighlight.vue";
import DropdownButton from "./DropdownButton.vue";
import HexViewer from "./HexViewer.vue";

export default {
  name: "HttpBody",
  components: {
    HexViewer,
    DropdownButton,
    AsyncHighlight,
    JsonNode,
    ClipboardCopyIcon,
    Button,
    Chips,
  },
  props: {
    bodyData: Object,
    requestUrl: String,
    show: Object,
    index: Number,
  },
  methods: {
    copyToClipboard(text, ref) {
      let el = this.$refs[ref];
      copyToClipboard(text, el);
    },
    formatResTypeChips(el)  {
      return el === 'preview'
          ? `preview ${this.resContentType}`
          : el;
    },
    viewRaw() {
      this.show.bodyType = 'raw';
    },
    collapseJson() {
      this.$refs.jsonNode.collapseAll()
    },
    expandJson() {
      this.$refs.jsonNode.expandAll()
    },
    string2Bin(str) {
      return new TextEncoder().encode(str);
    },
  },
  computed: {
    resHighlightContent() {
      return this.textData
          ? this.textData.trim()
          : 'REQUESTER-WARN: no body'
    },
    resContentType() {
      return findContentTypeInHeaders(this.bodyData.headers);
    },
    textData() {
      return this.bodyData.data || new TextDecoder().decode(this.byteData);
    },
    byteData() {
      return this.bodyData.data ? this.string2Bin(this.bodyData.data) : Uint8Array.fromBase64(this.bodyData.bytes);
    },
    imageDataUrl() {
      const uint8Array = new Uint8Array(this.byteData);
      const imageBlob = new Blob([uint8Array], { type: "image/png" });
      return URL.createObjectURL(imageBlob);
    },
    highlightLang() {
      const map = { json: "json", html: "xml", xml: "xml" };
      return map[this.resContentType] || null;
    },
    formattedPreview() {
      return this.show.bodyType.split("-").pop()
    },
    bodyViewActions() {
      const contentToActionMap = {
        "json"  : {"JSON":  () => {this.show.bodyType = "preview-json"}},
        "html"  : {"HTML":  () => {this.show.bodyType = "preview-html"}},
        "hex"   : {"HEX":   () => {this.show.bodyType = "preview-hex"}},
        "image" : {"IMAGE": () => {this.show.bodyType = "preview-image"}},
        ""      : {"TEXT":  () => {this.show.bodyType = "preview-text"}},
      };

      let out = {};

      for (const [key, value] of Object.entries(contentToActionMap)) {
        if (key !== this.resContentType) {
          for (const [subKey, subValue] of Object.entries(value)) {
            out[subKey] = subValue;
          }
        }
      }

      const primary = contentToActionMap[this.resContentType] || contentToActionMap[""];

      const temp = {};
      for (const [key, value] of Object.entries(primary)) {
        temp[key] = value;
      }
      out = {...temp, ...out};

      return out;
    },
    contentActions() {
      const contentToActionMap = {
        "preview-json"  : {"Collapse all": this.collapseJson, "Expand all": this.expandJson},
      };
      return contentToActionMap[this.show.bodyType] || null;
    }
  }
}
</script>
