<style lang="scss" scoped>
.bar {
  position: sticky;
  display: flex;
  gap: var(--gap-sm);
  align-items: center;
  padding-left: var(--gap-sm);
  z-index: 2;
  background-color: var(--color-button-bg);
  color: var(--dark-color-contrast);
  cursor: pointer;
  h3 {
    display: flex;
    gap: 10px;
    align-items: center;
  }
}

.big-bar {
  z-index: 5;
  top: 0;
  border-bottom: 1px solid var(--color-super-raised-bg);
  height: 45px;
  margin-bottom: var(--gap-sm);
}

.normal-bar {
  z-index: 4;
  top: 45px;
  height: 35px;
  border-bottom: 1px solid var(--color-raised-bg);
  padding-left: calc(var(--gap-sm) * 1.5);

  h4 {
    display: grid;
    grid-template-columns: 160px 4.5fr 150px;
    align-items: center;
    margin: 0;
    gap: 10px;
  }
  .time {
    position: absolute;
    right: 10px;
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
}

.mid-bar {
  z-index: 3;
  top: 75px;
  height: 28px;
  padding-left: calc(var(--gap-sm) * 2);
  border-bottom: 1px solid var(--color-raised-bg);
}

.small-bar {
  top: 45px;
  height: 25px;
  margin-bottom: var(--gap-sm);
  padding-left: calc(var(--gap-sm) * 2);
}

.tiny-bar {
  top: 100px;
  height: 25px;
  padding-left: calc(var(--gap-sm) * 3);
  border-bottom: 1px solid var(--color-raised-bg);
}

.url-holder:not(:empty) {
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

.url-input {
  display: flex;
  gap: var(--gap-sm);
  margin: var(--gap-sm) 0;

  input[type="text"] {
    width: 100%;
  }
}

.send-button {
  width: calc(100% - 20px);
  margin: 10px;
  text-align: center;
}

.section.headers,
.section.queries {
  display: flex;
  flex-flow: column nowrap;
  gap: var(--gap-sm);
  align-items: center;
  margin-bottom: var(--gap-sm);

  .header,
  .query {
    display: grid;
    grid-template-columns: 2fr 10fr 40px;
    gap: 8px;
  }

  .add-header,
  .add-query {
    width: 100%;
  }
}

.view-headers {
  display: block;
  margin: 0;
  padding: var(--gap-md);
  list-style: none;

  span {
    display: inline-block;
    font-weight: 600;
    min-width: 200px;
  }
}

.body {
  padding: var(--gap-md);

  iframe {
    width: 100%;
    margin-top: var(--gap-md);
    min-height: 500px;
    background-color: white;
  }

  pre {
    overflow: auto;
  }
}
.error {
  margin: var(--gap-sm);
  padding: var(--gap-sm);
  border-radius: var(--radius-sm);
  color: var(--color-red);
  border: 1px solid var(--color-red);
  font-size: 1.2rem;
  font-weight: 500;
}
</style>

<template>
  <div id="HttpRequestPage">
    <section>
      <div class="bar big-bar" @click="show.ReqForm = !show.ReqForm">
        <OpenArrowIcon :open="show.ReqForm" />
        <h3>
          HTTP request <span class="url-holder">{{ generateURL }}</span>
        </h3>
      </div>

      <div class="section" v-if="show.ReqForm">
        <div class="url-input">
          <DropdownSelect
            name="selectMethod"
            id="HTTP_METHOD_DROPDOWN"
            style="max-width: 13ch; height: 40px"
            :options="[
              'GET',
              'HEAD',
              'OPTIONS',
              'POST',
              'PUT',
              'PATCH',
              'DELETE',
              'TRACE',
            ]"
            v-model="method"
          />
          <input
            type="text"
            placeholder="Enter URL..."
            v-model="request.url"
            @keydown.enter="sendRequest"
          />
          <div
            class="btn icon-only"
            style="width: 50px; height: 40px"
            v-show="!validateUrl(generateURL) && generateURL !== ''"
            title="The URL is not valid!"
          >
            <ReportIcon />
          </div>
          <Button
            iconOnly
            style="width: 50px; height: 40px"
            title="Clear request form"
            @click="clearRequestForm"
            ><TrashIcon
          /></Button>
        </div>
        <section>
          <div
            class="bar small-bar"
            @click="show.ReqQueries = !show.ReqQueries"
          >
            <OpenArrowIcon :open="show.ReqQueries" />
            <h5>Request Queries ({{ request.queries.length }})</h5>
          </div>
          <div class="section queries" v-if="show.ReqQueries">
            <div
              v-for="(query, index) in request.queries"
              :key="index"
              class="query"
            >
              <input
                class="name"
                v-model="query.name"
                type="text"
                placeholder="Query Name..."
              />
              <input
                class="value"
                v-model="query.value"
                type="text"
                placeholder="Query Value..."
              />
              <Button
                @click="deleteQuery(index)"
                iconOnly
                style="height: 40px; width: 40px"
              >
                <TrashIcon />
              </Button>
            </div>
            <Button @click="addQuery" iconOnly class="add-query">
              <PlusIcon />
            </Button>
          </div>
        </section>
        <section>
          <div
            class="bar small-bar"
            @click="show.ReqHeaders = !show.ReqHeaders"
          >
            <OpenArrowIcon :open="show.ReqHeaders" />
            <h5>Request Headers ({{ request.headers.length }})</h5>
          </div>
          <div class="section headers" v-if="show.ReqHeaders">
            <div
              v-for="(header, index) in request.headers"
              :key="index"
              class="header"
            >
              <input
                class="name"
                v-model="header.name"
                type="text"
                placeholder="Header Name..."
              />
              <input
                class="value"
                v-model="header.value"
                type="text"
                placeholder="Header Value..."
              />
              <Button
                @click="deleteHeader(index)"
                iconOnly
                style="height: 40px; width: 40px"
              >
                <TrashIcon />
              </Button>
            </div>
            <Button @click="addHeader" iconOnly class="add-header">
              <PlusIcon />
            </Button>
          </div>
        </section>
        <section>
          <div class="bar small-bar" @click="show.ReqBody = !show.ReqBody">
            <OpenArrowIcon :open="show.ReqBody" />
            <h5>Request Body ({{ request.body.length }})</h5>
          </div>
          <div class="section body" v-if="show.ReqBody">
            <BodyBuilder
              style="
                min-height: 200px;
                width: 100%;
                max-width: calc(100% - 10px);
                margin: 5px 5px;
              "
              :headers="request.headers"
              v-model="request.body"
            ></BodyBuilder>
          </div>
        </section>
        <Button
          @click="sendRequest"
          color="secondary"
          :disabled="this.generateURL == '' || requestInProgress"
          class="send-button"
        >
          <SendIcon />Send Request
        </Button>
      </div>
    </section>
    <!-- ################################### Responses ################################### -->
    <section>
      <div class="bar big-bar" @click="show.Responses = !show.Responses">
        <OpenArrowIcon :open="show.Responses" />
        <h3 style="width: 100%">Responses ({{ responses.length }})</h3>
        <Button @click="clearHistory"><TrashIcon />Clear</Button>
      </div>
      <div class="section" v-if="show.Responses">
        <div v-for="(item, index) in responses" :key="index">
          <div
            class="bar normal-bar"
            @click="show.Response[index].open = !show.Response[index].open"
          >
            <OpenArrowIcon :open="show.Response[index].open" />
            <h4>
              <span> {{ item.request.method }} - Request </span>
              <span class="url"> {{ item.request.url }}</span>

              <span class="time">
                {{
                  item.time !== null
                    ? item.time !== -2
                      ? item.time + "ms"
                      : "Error "
                    : "waiting..."
                }}
                |
                <span
                  :style="{
                    color: `var(--status-${item.response.status_code}-color)`,
                  }"
                >
                  {{
                    item.response.status_code == -1
                      ? "Error"
                      : item.response.status_code
                  }}
                </span></span
              >
            </h4>
          </div>
          <div v-if="show.Response[index].open">
            <div>
              <div
                class="mid-bar bar"
                @click="
                  show.Response[index].request.all =
                    !show.Response[index].request.all
                "
              >
                <OpenArrowIcon :open="show.Response[index].request.all" />
                <h4>Request Info</h4>
              </div>
              <div v-if="show.Response[index].request.all">
                <HttpRequestInfo
                  :index="index"
                  :item="item"
                  @copyRequest="copyRequest"
                />
                <div v-if="item.request.headers.length > 0">
                  <div
                    class="bar tiny-bar"
                    @click="
                      show.Response[index].request.headers =
                        !show.Response[index].request.headers
                    "
                  >
                    <OpenArrowIcon
                      :open="show.Response[index].request.headers"
                    />
                    <h5>Request headers</h5>
                  </div>
                  <div
                    v-if="show.Response[index].request.headers"
                    class="section req-headers"
                  >
                    <ul class="view-headers">
                      <li
                        v-for="(header, index) in item.request.headers"
                        :key="index"
                      >
                        <span
                          :style="`color:var(--status-${item.response.status_code}-color)`"
                          >{{ header.name }}:</span
                        >
                        {{ header.value }}
                      </li>
                    </ul>
                  </div>
                </div>
                <div v-if="item.request.data">
                  <div
                    class="bar tiny-bar"
                    @click="
                      show.Response[index].request.body =
                        !show.Response[index].request.body
                    "
                  >
                    <OpenArrowIcon :open="show.Response[index].request.body" />
                    <h5>Request body</h5>
                  </div>
                  <div
                    v-if="show.Response[index].request.body"
                    class="section body"
                  >
                    {{ item.request.data }}
                  </div>
                </div>
              </div>
            </div>
            <div>
              <div
                class="bar mid-bar"
                @click="
                  show.Response[index].response.all =
                    !show.Response[index].response.all
                "
              >
                <OpenArrowIcon :open="show.Response[index].response.all" />
                <h4>Response</h4>
              </div>
              <div v-if="show.Response[index].response.all">
                <div v-if="!item.response.error">
                  <div
                    v-if="
                      item.response.headers && item.response.headers.length > 0
                    "
                  >
                    <div
                      class="bar tiny-bar"
                      @click="
                        show.Response[index].response.headers =
                          !show.Response[index].response.headers
                      "
                    >
                      <OpenArrowIcon
                        :open="show.Response[index].response.headers"
                      />
                      <h5>Response Headers</h5>
                    </div>
                    <ul
                      class="view-headers"
                      v-if="show.Response[index].response.headers"
                    >
                      <li
                        v-for="(header, index) in item.response.headers"
                        :key="index"
                      >
                        <span
                          :style="`color:var(--status-${item.response.status_code}-color)`"
                          >{{ header.name }}:</span
                        >
                        {{ header.value }}
                      </li>
                    </ul>
                  </div>
                  <div>
                    <div
                      class="bar tiny-bar"
                      @click="
                        show.Response[index].response.body =
                          !show.Response[index].response.body
                      "
                    >
                      <OpenArrowIcon
                        :open="show.Response[index].response.body"
                      />
                      <h5>Response Body</h5>
                    </div>
                    <div
                      class="body"
                      v-if="
                        show.Response[index].response.body &&
                        item.response.status_code
                      "
                    >
                      <Button
                        style="float: right"
                        @click="
                          copyToClipboard(
                            item.response.result,
                            'copyButton' + index
                          )
                        "
                        ><ClipboardCopyIcon />
                        <span :ref="'copyButton' + index">Copy</span></Button
                      >
                      <div
                        v-if="findContentType(item.response.headers) !== 'text'"
                      >
                        <Chips
                          v-model="show.Response[index].response.bodyType"
                          :items="['raw', 'preview']"
                          :formatLabel="
                            (el) => {
                              return el === 'preview'
                                ? `preview ${findContentType(
                                    item.response.headers
                                  )}`
                                : el;
                            }
                          "
                        />
                      </div>
                      <div
                        v-if="
                          findContentType(item.response.headers) == 'html' &&
                          show.Response[index].response.bodyType == 'preview'
                        "
                      >
                        <iframe
                          :srcdoc="item.response.result"
                          frameborder="0"
                        ></iframe>
                      </div>
                      <div
                        v-if="
                          findContentType(item.response.headers) == 'image' &&
                          show.Response[index].response.bodyType == 'preview'
                        "
                      >
                        <img
                          style="margin: 10px; width: unset; height: unset"
                          :src="item.request.url"
                        />
                      </div>
                      <div
                        v-else-if="
                          findContentType(item.response.headers) == 'json' &&
                          show.Response[index].response.bodyType == 'preview'
                        "
                      >
                        <strong
                          >Json parser is not done, if you wish to have this
                          feature, contribute it on github</strong
                        >
                        <div>
                          <JsonHighlight
                            :data="prettifyJson(item.response.result)"
                          />
                        </div>
                      </div>
                      <div v-else>
                        <highlightjs
                          autodetect
                          :code="
                            item.response.result
                              ? item.response.result.trim()
                              : 'REQUESTER-WARN: no body'
                          "
                          class="response-body"
                        ></highlightjs>
                      </div>
                    </div>
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
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>
<script>
import {
  DropdownSelect,
  Button,
  TrashIcon,
  PlusIcon,
  SendIcon,
  ClipboardCopyIcon,
  Chips,
  ReportIcon,
} from "omorphia";
import { invoke } from "@tauri-apps/api/tauri";
import OpenArrowIcon from "../components/icons/OpenArrowIcon.vue";
import JsonHighlight from "../components/JsonHighlight.vue";
import HttpRequestInfo from "../components/HttpRequestInfo.vue";
import BodyBuilder from "../components/BodyBuilder.vue";
import {
  defaultOpenResponseTabs,
  defaultClosedResponseTabs,
  findContentTypeInHeaders,
  parseUrlToQueries,
  validateUrl,
} from "../utils/http";
import { copyToClipboard, deepCopy } from "../utils/simple";
export default {
  components: {
    OpenArrowIcon,
    DropdownSelect,
    Button,
    TrashIcon,
    PlusIcon,
    SendIcon,
    ClipboardCopyIcon,
    Chips,
    JsonHighlight,
    HttpRequestInfo,
    BodyBuilder,
    ReportIcon,
  },
  data() {
    return {
      method: "GET",
      show: {
        ReqForm: true,
        ReqQueries: true,
        ReqHeaders: false,
        ReqBody: false,
        Responses: true,
        Response: [],
      },
      request: {
        url: "",
        queries: [],
        headers: [],
        body: "",
      },
      responses: [],
      config: {},
      requestInProgress: false,
    };
  },
  mounted() {
    this.loadConfig();
    this.loadHistory();
  },
  methods: {
    addQuery() {
      this.request.queries.push({ name: "", value: "" });
    },
    deleteQuery(index) {
      this.request.queries.splice(index, 1);
    },
    addHeader() {
      this.request.headers.push({ name: "", value: "" });
    },
    deleteHeader(index) {
      this.request.headers.splice(index, 1);
    },
    async sendRequest() {
      if (this.generateURL !== "") {
        let request = {
          method: this.method,
          url: this.generateURL,
          data: this.request.body,
          headers: this.request.headers.map((header) => [
            header.name,
            header.value,
          ]),
        };
        let requestInfo = {
          request: request,
          response: {},
          time: null,
        };
        this.responses.unshift(requestInfo);
        this.show.Response.unshift(defaultOpenResponseTabs);
        console.log("Sending request", request);
        this.requestInProgress = true;
        try {
          let response = await invoke("send_request", request);
          this.responses[0] = response;
          this.loadHistory().then(() => {
            this.show.Response[0].open = true;
          });
        } catch (e) {
          this.responses[0] = {
            request: request,
            response: { error: e, status_code: -1 },
            time: -2,
          };
        } finally {
          this.requestInProgress = false;
          if (this.show.Response[1]) {
            this.show.Response[1].open = false;
          }
        }
      }
    },
    copyToClipboard(text, ref) {
      let el = this.$refs[ref];
      copyToClipboard(text, el);
    },
    findContentType(headers) {
      return findContentTypeInHeaders(headers);
    },
    async loadHistory() {
      let history = await invoke("read_http_history");
      history = JSON.parse(history);
      history.forEach(() => {
        this.show.Response.unshift(deepCopy(defaultClosedResponseTabs));
      });
      this.responses = history;
    },
    async clearHistory() {
      let returns = await invoke("clear_http_history");
      if (!returns) {
        this.show.Responses = true;
        this.loadHistory();
      }
    },
    copyRequest(index) {
      let requestInfo = this.responses[index].request;
      this.request.headers = requestInfo.headers;
      this.request.body = requestInfo.data;
      if (requestInfo.url.includes("?")) {
        this.request.url = requestInfo.url.split("?")[0];
        this.request.queries = this.parseQueries(requestInfo.url);
      } else {
        this.request.queries = [];
        this.request.url = requestInfo.url;
      }
      this.method = requestInfo.method;
    },
    clearRequestForm() {
      this.request.headers = [];
      this.request.body = "";
      this.request.url = "";
      this.loadConfig();
    },
    prettifyJson(data) {
      try {
        return JSON.stringify(JSON.parse(data), null, 2);
      } catch (error) {
        console.error("Error parsing JSON:", error);
        return "Invalid JSON data\n" + data;
      }
    },
    highlightCode(code, lang) {
      getHighlighter({
        theme: "nord",
      }).then((highlighter) => {
        return highlighter.codeToHtml(code, { lang: lang });
      });
    },
    parseQueries(url) {
      return parseUrlToQueries(url);
    },
    async loadConfig() {
      let config = await invoke("get_config_values");
      this.config = JSON.parse(config);
      if (config) {
        this.request.headers = this.config.defaults.headers;
        this.request.queries = this.config.defaults.queries;
        this.request.body = this.config.defaults.body;
      }
    },
    validateUrl(url) {
      return validateUrl(url);
    },
  },
  computed: {
    generateURL() {
      if (!this.request.url) {
        return "";
      }

      let formattedURL = this.request.url;
      if (!/^https?:\/\//i.test(this.request.url)) {
        formattedURL = `http://${this.request.url}`;
      }

      const queryParams = this.request.queries
        .filter((query) => query.name && query.value)
        .map((query) => `${query.name}=${query.value}`)
        .join("&");

      return `${formattedURL}${queryParams ? "?" + queryParams : ""}`;
    },
  },
};
</script>
