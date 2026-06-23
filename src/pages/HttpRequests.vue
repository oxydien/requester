<style lang="scss" scoped>
#HttpRequestPage {
  display: flex;
  flex-flow: column nowrap;
  gap: var(--gap-md);
}

.header-title {
  display: flex;
  gap: 10px;
  width: 100%;
  align-items: center;
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
  margin: var(--gap-sm);

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
  margin: var(--gap-sm);

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

  .query-add-options {
    width: 100%;
  }
}

.qol-min-height {
  min-height: 50vh;
}
</style>

<template>
  <div id="HttpRequestPage">
    <section id="HTTP_FORM_SECTION">
      <CollapsableBar kind="big" v-model:modelValue="show.ReqForm">
        <template #header>
          <h3 class="header-title">
            Create a request <span class="url-holder">{{ generateURL }}</span>
          </h3>
          <Button color="danger" outline @click.stop="clearRequestForm">
            <TrashIcon />
            Clear
          </Button>
        </template>

        <template #content>
          <div class="section">
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
            </div>
            <section id="HttpRequestQueries">
              <CollapsableBar kind="small" v-model:modelValue="show.ReqQueries">
                <template #header>
                  <h5>Request Queries ({{ request.queries.length }})</h5>
                </template>
                <template #content>
                  <div class="section queries">
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
                    <div class="just-flex query-add-options">
                      <Button @click="addQuery" iconOnly class="add-query">
                        <PlusIcon />
                      </Button>
                      <Button @click="importQueries" v-if="originalUrlContainsQuery" class="add-query">
                        <UploadIcon /> Take from Url
                      </Button>
                    </div>
                  </div>
                </template>
              </CollapsableBar>
            </section>
            <section id="HttpRequestHeaders">
              <CollapsableBar kind="small" v-model:modelValue="show.ReqHeaders">
                <template #header>
                  <h5>Request Headers ({{ request.headers.length }})</h5>
                </template>
                <template #content>
                  <div class="section headers">
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
                </template>
              </CollapsableBar>
            </section>
            <section id="HttpRequestBody">
              <CollapsableBar kind="small" v-model:modelValue="show.ReqBody">
                <template #header>
                  <h5>Request Body ({{ request.body.length }})</h5>
                </template>
                <template #content>
                  <div class="section body">
                    <BodyBuilder
                        style="
                min-height: 200px;
                width: 100%;
                max-width: calc(100% - 10px);
                margin: 5px 5px;
              "
                        :headers="request.headers"
                        v-model:modelValue="request.body"
                        v-model:headers="request.headers"
                    ></BodyBuilder>
                  </div>
                </template>
              </CollapsableBar>
            </section>
            <Button
                @click="sendRequest"
                color="secondary"
                :disabled="this.generateURL === '' || requestInProgress"
                class="send-button"
            >
              <SendIcon />Send Request
            </Button>
          </div>
        </template>
      </CollapsableBar>
    </section>
    <section id="RESPONSES_SECTION">
      <CollapsableBar kind="big" v-model:modelValue="show.Responses">
        <template #header>
          <h3 style="width: 100%">Responses ({{ responses.length }})</h3>
          <Button color="danger" outline @click.stop="clearHistory"><TrashIcon />Clear</Button>
        </template>
        <template #content>
            <HttpResponse
                v-for="(item, index) in responses"
                :key="index"
                :index="index"
                :show="show.Response[index]"
                :item="item"
                @copy-request="copyRequest"
            />
        </template>
      </CollapsableBar>
    </section>
    <div class="qol-min-height"></div>
  </div>
</template>
<script>
import {Button, Chips, ClipboardCopyIcon, DropdownSelect, PlusIcon, UploadIcon, ReportIcon, SendIcon, TrashIcon,} from "omorphia";
import {invoke} from "@tauri-apps/api/core";
import OpenArrowIcon from "../components/icons/OpenArrowIcon.vue";
import JsonHighlight from "../components/JsonHighlight.vue";
import HttpRequestInfo from "../components/HttpRequestInfo.vue";
import BodyBuilder from "../components/BodyBuilder.vue";
import {defaultClosedResponseTabs, defaultOpenResponseTabs, parseUrlToQueries, validateUrl,} from "../utils/http";
import {deepCopy} from "../utils/simple";
import HttpResponse from "../components/HttpResponse.vue";
import CollapsableBar from "../components/CollapsableBar.vue";

export default {
  components: {
    CollapsableBar,
    HttpResponse,
    OpenArrowIcon,
    UploadIcon,
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
          this.responses[0] = await invoke("send_request", request);
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
        this.show.Response = [];
        await this.loadHistory();
      }
    },
    copyRequest(index, features) {
      let copyFeatures = features || ["url", "queries", "headers", "body"];

      let requestInfo = this.responses[index].request;
      if (!requestInfo) {
        console.error("Could not copy request", index, features, this.responses);
        return;
      }

      if (requestInfo.url.includes("?")) {
        if (copyFeatures.includes("url"))
          this.request.url = requestInfo.url.split("?")[0];

        if (copyFeatures.includes("queries"))
          this.request.queries = this.parseQueries(requestInfo.url);
      } else {
        if (copyFeatures.includes("url"))
          this.request.url = requestInfo.url;

        if (copyFeatures.includes("queries"))
          this.request.queries = [];
      }

      if (copyFeatures.includes("headers"))
        this.request.headers = requestInfo.headers;

      if (copyFeatures.includes("body"))
        this.request.body = requestInfo.data;

      this.method = requestInfo.method;
    },
    clearRequestForm() {
      this.request.headers = [];
      this.request.body = "";
      this.request.url = "";
      this.loadConfig();
    },
    parseQueries(url) {
      return parseUrlToQueries(url);
    },
    importQueries() {
      this.request.queries.push(...this.parseQueries(this.request.url));
      this.request.url = this.request.url.split("?")[0];
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
        .filter((query) => query.name)
        .map((query) => query.value ? `${query.name}=${query.value}` : query.name)
        .join("&");

      return `${formattedURL}${queryParams ? "?" + queryParams : ""}`;
    },
    originalUrlContainsQuery() {
      return this.request.url.includes("?");
    }
  },
};
</script>
