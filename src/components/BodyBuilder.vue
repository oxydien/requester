<style lang="scss" scoped>
#bodyBuilder {
  position: relative;
  .overlay {
    position: absolute;
    right: 0;
    margin: 10px;
    max-width: 50%;
    .true {
      display: flex;
      align-items: center;
    }
    .xml,
    .json {
      display: flex;
      flex-flow: column;
      align-items: flex-end;
      .message {
        display: none;
      }
      &:hover {
        .message {
          display: block;
        }
      }
    }
  }
}
</style>

<template>
  <div id="bodyBuilder">
    <div class="overlay">
      <div class="json">
        <div
          class="true"
          :style="
            isBodyJson !== true && value !== ''
              ? 'cursor: help'
              : 'cursor: pointer'
          "
          @click="toggleHeader('json')"
        >
          <span ref="span_json_Message">JSON</span>
          <CheckIcon v-if="isBodyJson === true" style="color: lime" /><XIcon
            v-else
            style="color: red"
          />
        </div>
        <div class="message" v-if="isBodyJson !== true && value !== ''">
          {{ isBodyJson }}
        </div>
      </div>
      <div class="xml">
        <div
          class="true"
          :style="
            isBodyXml !== true && value !== ''
              ? 'cursor: help'
              : 'cursor: pointer'
          "
          @click="toggleHeader('xml')"
        >
          <span ref="span_xml_Message">XML</span>
          <CheckIcon v-if="isBodyXml === true" style="color: lime" /><XIcon
            v-else
            style="color: red"
          />
        </div>
        <div class="message" v-if="isBodyXml !== true && value !== ''">
          {{ isBodyXml }}
        </div>
      </div>
      <!--  bin, ... here -->
    </div>
    <textarea
      class="body"
      v-model="value"
      placeholder="Request body..."
      :style="style"
      @input="change"
    >
    </textarea>
  </div>
</template>

<script>
import { CheckIcon, XIcon } from "omorphia";
export default {
  data() {
    return {
      value: "",
      isBodyJson: false,
      isBodyXml: false,
    };
  },
  components: {
    CheckIcon,
    XIcon,
  },
  props: {
    headers: {
      type: Array,
      required: true,
    },
    modelValue: {
      type: String,
      required: true,
    },
    style: {
      type: Object,
      default: "",
    },
  },
  emits: ["update:modelValue", "update:headers"],
  mounted() {
    this.value = this.modelValue;
    this.isBodyJson = this.checkForJson();
    this.isBodyXml = this.checkForXML();
  },
  methods: {
    change() {
      this.$emit("update:modelValue", this.value);
      this.isBodyJson = this.checkForJson();
      this.isBodyXml = this.checkForXML();
    },
    checkForJson() {
      try {
        JSON.parse(this.value);
        return true;
      } catch (e) {
        return e.toString().substring(13);
      }
    },
    checkForXML() {
      const parser = new DOMParser();
      try {
        const xmlDoc = parser.parseFromString(this.value, "text/xml");
        if (xmlDoc.getElementsByTagName("parsererror").length > 0) {
          const errorElement = xmlDoc.getElementsByTagName("parsererror")[0];
          return errorElement.textContent.trim();
        } else {
          return true;
        }
      } catch (e) {
        return e;
      }
    },
    toggleHeader(type) {
      const contentTypeHeaderIndex = this.headers.findIndex(
        (header) => header.name.toLowerCase() === "content-type"
      );
      const spanMessage = this.$refs[`span_${type}_Message`];

      if (contentTypeHeaderIndex !== -1) {
        // you could theoretically splice the content-type header out...
        spanMessage.innerText = "There is Content-Type already";
        setTimeout(() => {
          spanMessage.innerText = type.toUpperCase();
        }, 3000);
      } else {
        this.headers.push({
          name: "content-type",
          value: "application/" + type,
        });
        spanMessage.innerText = `Added ${type} Content-Type header`;
        setTimeout(() => {
          spanMessage.innerText = type.toUpperCase();
        }, 3000);
      }
    },
  },
};
</script>
