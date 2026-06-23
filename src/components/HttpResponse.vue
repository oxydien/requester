<style scoped lang="scss">
.header {
  display: flex;
  align-items: center;
  margin: 0;
  gap: 10px;
  width: calc(100% - 35px);
  font-weight: 700;

  .type {
    min-width: 150px;
  }

  .status {
    min-width: 4ch;
  }

  .url {
    display: block;
    font-weight: 300;
    color: var(--dark-color-base);
    background-color: #00000077;
    padding: 2px;
    border-radius: 5px;
    max-width: 70vw;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .result {
    display: flex;
    flex-flow: row-reverse nowrap;
    gap: 4px;
    min-width: 120px;
    flex-grow: 1;
    justify-content: space-between;
  }
}
</style>

<template>
  <CollapsableBar kind="normal" v-model:modelValue="show.open">
    <template #header v-if="item.request.method">
      <div class="header">
        <span class="type">{{ item.request.method }} - Request</span>
        <span class="status" :style="statusStyle">{{ statusText }} ‧</span>
        <span class="url">{{ item.request.url }}</span>
        <div class="result">
          <span class="time">
            {{ requestTime }}
          </span>
        </div>
      </div>
    </template>

    <template #content>
      <div>
        <CollapsableBar kind="mid" v-model:modelValue="show.request.all">
          <template #header>
            <SendIcon /><h4>Request Info</h4>
          </template>
          <template #content>
            <HttpRequestInfo
                :index="index"
                :item="item"
                @copyRequest="copyRequest"
            />
            <template v-if="item.request.headers.length > 0">
              <CollapsableBar kind="tiny" v-model:modelValue="show.request.headers">
                <template #header>
                  <h5>Request headers</h5>
                </template>

                <template #content>
                  <HttpHeadersView :color="statusStyleColor" :headers="item.request.headers" />
                </template>
              </CollapsableBar>
            </template>

            <template v-if="item.request.data">
              <CollapsableBar kind="tiny" v-model:modelValue="show.request.data">
                <template #header>
                  <h5>Request body</h5>
                </template>
                <template #content>
                  <HttpBody :bodyData="item.request" :requestUrl="item.request.url" :show="show.request" :index="index" />
                </template>
              </CollapsableBar>
            </template>
          </template>
        </CollapsableBar>
      </div>

      <div>
        <CollapsableBar kind="mid" v-model:modelValue="show.response.all">
          <template #header>
            <MailIcon /><h4>Response</h4>
          </template>

          <template #content>
            <div v-if="!item.response.error">
              <div v-if="item.response.headers && item.response.headers.length > 0">
                <CollapsableBar kind="tiny" v-model:modelValue="show.response.headers">
                  <template #header>
                    <h5>Response Headers</h5>
                  </template>
                  <template #content>
                    <HttpHeadersView :color="statusStyleColor" :headers="item.response.headers" />
                  </template>
                </CollapsableBar>
              </div>
              <div>
                <CollapsableBar kind="tiny" v-model:modelValue="show.response.body">
                  <template #header>
                    <h5>Response Body</h5>
                  </template>
                  <template #content v-if="item.response.status_code">
                    <HttpBody :bodyData="item.response" :requestUrl="item.request.url" :show="show.response" :index="index" />
                  </template>
                </CollapsableBar>
              </div>
            </div>
            <div v-else class="error">
              {{ item.response.error }}
              <Button
                  style="margin: 0 0 0 auto"
                  outline
                  @click="
                      copyToClipboard(
                        item.response.error,
                        'errorCopyButton' + index
                      )
                    "
              ><ClipboardCopyIcon />
                <span :ref="'errorCopyButton' + index">Copy</span></Button
              >
            </div>
          </template>
        </CollapsableBar>
      </div>
    </template>
  </CollapsableBar>
</template>

<script>
import HttpRequestInfo from "./HttpRequestInfo.vue";
import JsonHighlight from "./JsonHighlight.vue";
import OpenArrowIcon from "./icons/OpenArrowIcon.vue";
import {Button, Chips, ClipboardCopyIcon, SendIcon, MailIcon} from "omorphia";
import CollapsableBar from "./CollapsableBar.vue";
import {findContentTypeInHeaders} from "../utils/http.js";
import {copyToClipboard} from "../utils/simple.js";
import JsonNode from "./JsonNode.vue";
import HttpHeadersView from "./HttpHeadersView.vue";
import HttpBody from "./HttpBody.vue";

export default {
  name: "HttpResponse",
  components: {
    HttpBody,
    HttpHeadersView,
    JsonNode,
    CollapsableBar,
    OpenArrowIcon,
    Button,
    Chips,
    JsonHighlight,
    HttpRequestInfo,
    ClipboardCopyIcon,
    SendIcon, MailIcon,
  },
  props: {
    item: { // { request, response, time }
      type: Object,
      required: true,
    },
    show: {
      type: Object, // { open, request, response }
      required: true,
    },
    index: {
      type: Number,
      required: true,
    }
  },
  emits: ["copyRequest"],
  methods: {
    copyRequest(index, features) {
      this.$emit("copyRequest", index, features);
    },
    copyToClipboard(text, ref) {
      let el = this.$refs[ref];
      copyToClipboard(text, el);
    },
  },
  computed: {
    requestTime() {
      if (this.item.time != null) {
        if (this.item.time !== -2) {
          return this.item.time + "ms";
        }
        return "Error"
      }
      return "waiting..."
    },
    statusStyle() {
      return { color: this.statusStyleColor }
    },
    statusStyleColor() {
      return `var(--status-${this.item.response.status_code}-color)`
    },
    statusText() {
      if (this.item.response.status_code === -1) {
        return "Error";
      }
      return this.item.response.status_code
    },
    resContentType() {
      return findContentTypeInHeaders(this.item.response.headers);
    }
  }
}
</script>