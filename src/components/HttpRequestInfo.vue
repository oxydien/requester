<style lang="scss" scoped>
.section.request-info {
  padding: 0.5rem;
  p {
    line-height: 1.2;
    margin: 2px;
    strong {
      display: inline-block;
      min-width: 150px;
    }
  }
}
</style>

<template>
  <div class="section request-info">
    <Button @click="copyRequest(index)" style="float: right"
      ><ClipboardCopyIcon /> Copy request</Button
    >
    <p>
      <strong :style="`color:var(--status-${item.response.status_code}-color)`"
        >Method:
      </strong>
      <span>{{ item.request.method }}</span>
    </p>
    <p>
      <strong :style="`color:var(--status-${item.response.status_code}-color)`"
        >Url: </strong
      ><span>{{ item.request.url }}</span>
    </p>
    <p>
      <strong :style="`color:var(--status-${item.response.status_code}-color)`"
        >Response status:
      </strong>
      <span
        >{{ item.response.status_code }} :
        {{ statusCodes[item.response.status_code] }}</span
      >
    </p>
  </div>
</template>
<script>
import { Button, ClipboardCopyIcon } from "omorphia";
export default {
  components: {
    Button,
    ClipboardCopyIcon,
  },
  props: {
    item: {
      required: true,
      type: Object,
    },
    index: {
      required: true,
      type: Number,
    },
  },
  emits: ["copyRequest"],
  methods: {
    copyRequest(index) {
      this.$emit("copyRequest", index);
    },
  },
  computed: {
    statusCodes() {
      return {
        "-1": "Internal/External Program Error",
        100: "Continue",
        101: "Switching Protocols",
        102: "Processing",
        103: "Early Hints",
        200: "OK",
        201: "Created",
        202: "Accepted",
        203: "Non-Authoritative Information",
        204: "No Content",
        205: "Reset Content",
        206: "Partial Content",
        207: "Multi-Status",
        208: "Already Reported",
        226: "IM Used",
        300: "Multiple Choices",
        301: "Moved Permanently",
        302: "Found",
        303: "See Other",
        304: "Not Modified",
        305: "Use Proxy",
        307: "Temporary Redirect",
        308: "Permanent Redirect",
        400: "Bad Request",
        401: "Unauthorized",
        402: "Payment Required",
        403: "Forbidden",
        404: "Not Found",
        405: "Method Not Allowed",
        406: "Not Acceptable",
        407: "Proxy Authentication Required",
        408: "Request Timeout",
        409: "Conflict",
        410: "Gone",
        411: "Length Required",
        412: "Precondition Failed",
        413: "Payload Too Large",
        414: "URI Too Long",
        415: "Unsupported Media Type",
        416: "Range Not Satisfiable",
        417: "Expectation Failed",
        418: "I'm a Teapot",
        421: "Misdirected Request",
        422: "Unprocessable Entity",
        423: "Locked",
        424: "Failed Dependency",
        425: "Too Early",
        426: "Upgrade Required",
        428: "Precondition Required",
        429: "Too Many Requests",
        431: "Request Header Fields Too Large",
        451: "Unavailable For Legal Reasons",
        500: "Internal Server Error",
        501: "Not Implemented",
        502: "Bad Gateway",
        503: "Service Unavailable",
        504: "Gateway Timeout",
        505: "HTTP Version Not Supported",
        506: "Variant Also Negotiates",
        507: "Insufficient Storage",
        508: "Loop Detected",
        510: "Not Extended",
        511: "Network Authentication Required",
      };
    },
  },
};
</script>
