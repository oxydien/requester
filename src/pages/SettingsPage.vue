<style lang="scss" scoped>
#settingsPage {
  position: relative;
  padding: 1rem;
  .heading {
    position: sticky;
    top: -1rem;
    margin: -1rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 45px;
    font-size: 1.8rem;
    font-weight: 600;
    padding: 0 1rem;
    background-color: var(--color-button-bg);
    border-bottom: 1px solid var(--color-bg);
    z-index: 56;

    .nav {
      display: flex;
      gap: var(--gap-sm);
    }
  }
  .http-url {
    display: flex;
    gap: var(--gap-sm);
    margin: var(--gap-sm) 0;

    input[type="text"] {
      width: 100%;
    }
  }
  .bar {
    position: sticky;
    top: 28px;
    display: flex;
    align-items: center;
    gap: var(--gap-sm);
    margin: 0 -15px;
    padding: 5px 15px;
    background-color: var(--color-button-bg);
    border-bottom: 1px solid var(--color-bg);
    cursor: pointer;
    z-index: 3;
  }
  .small-bar {
    height: 25px;
    top: 60px;
    font-size: 0.9rem;
    z-index: 2;
  }
}
.section.headers,
.section.queries {
  display: flex;
  flex-flow: column nowrap;
  gap: var(--gap-sm);
  align-items: center;
  margin: var(--gap-sm) 0;

  .header,
  .query {
    width: 100%;
    display: grid;
    grid-template-columns: 3fr 10fr 40px;
    gap: 8px;
  }

  .add-header,
  .add-query {
    width: 100%;
  }
}
</style>

<template>
  <div id="settingsPage">
    <div class="heading">
      <span style="display: flex; align-items: center; gap: var(--gap-sm)">
        <SettingsIcon />
        Settings
      </span>
      <div class="nav">
        <Button @click="openConfig" color="secondary" iconOnly
          ><CodeIcon
        /></Button>
        <Button
          color="highlight"
          :disabled="JSON.stringify(config) == JSON.stringify(previousConfig)"
          @click="saveConfig"
          iconOnly
          ><SaveIcon
        /></Button>
        <Button @click="loadConfig" color="danger" outline iconOnly>
          <ReloadIcon />
        </Button>
      </div>
    </div>
    <section style="margin-top: 18px">
      <h3 class="bar" @click="open.http.open = !open.http.open">
        <OpenArrowIcon :open="open.http.open" />Http defaults
      </h3>
      <div class="http defaults" v-show="open.http.open">
        <div class="http-url">
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
            v-model="config.http.defaults.method"
          />
          <input
            type="text"
            placeholder="Enter URL..."
            v-model="config.http.defaults.url"
          />
          <input
            type="number"
            placeholder="History limit..."
            title="History limit"
            style="max-width: 130px"
            v-model="config.http.history.save_amount"
          />
        </div>
        <h3
          class="bar small-bar"
          @click="open.http.queries = !open.http.queries"
        >
          <OpenArrowIcon :open="open.http.queries" />
          Default Http Request Queries ({{
            config.http.defaults.queries.length
          }})
        </h3>
        <div class="section queries" v-show="open.http.queries">
          <div
            v-for="(query, index) in config.http.defaults.queries"
            :key="index"
            class="query"
          >
            <input
              class="name"
              v-model="query.name"
              type="text"
              style="width: 100%"
              placeholder="Query Name..."
            />
            <input
              class="value"
              v-model="query.value"
              type="text"
              style="width: 100%"
              placeholder="Query Value..."
            />
            <Button @click="deleteHttpQuery(index)" iconOnly>
              <TrashIcon />
            </Button>
          </div>
          <Button @click="addHttpQuery()" iconOnly class="add-query">
            <PlusIcon />
          </Button>
        </div>
        <h3
          class="bar small-bar"
          @click="open.http.headers = !open.http.headers"
        >
          <OpenArrowIcon :open="open.http.headers" />
          Default Http Request Headers ({{
            config.http.defaults.headers.length
          }})
        </h3>
        <div class="section headers" v-show="open.http.headers">
          <div
            v-for="(header, index) in config.http.defaults.headers"
            :key="index"
            class="header"
          >
            <input
              class="name"
              v-model="header.name"
              type="text"
              style="width: 100%"
              placeholder="Header Name..."
            />
            <input
              class="value"
              v-model="header.value"
              type="text"
              style="width: 100%"
              placeholder="Header Value..."
            />
            <Button @click="deleteHttpHeader(index)" iconOnly>
              <TrashIcon />
            </Button>
          </div>
          <Button @click="addHttpHeader" iconOnly class="add-header">
            <PlusIcon />
          </Button>
        </div>

        <h3 class="bar small-bar" @click="open.http.body = !open.http.body">
          <OpenArrowIcon :open="open.http.body" />
          Default Http Request Body ({{ config.http.defaults.body.length }})
        </h3>
        <BodyBuilder
          :style="{
            'min-height': '200px',
            'min-width': 'calc(100% - 10px)',
            'max-width': 'calc(100% - 10px)',
            margin: '5px 5px',
          }"
          :headers="config.http.defaults.headers"
          v-model="config.http.defaults.body"
          v-show="open.http.body"
        ></BodyBuilder>
      </div>
    </section>
  </div>
</template>
<script>
import { invoke } from "@tauri-apps/api/tauri";
import {
  Button,
  TrashIcon,
  PlusIcon,
  Card,
  SaveIcon,
  SettingsIcon,
  CodeIcon,
  DropdownSelect,
} from "omorphia";
import OpenArrowIcon from "../components/icons/OpenArrowIcon.vue";
import ReloadIcon from "../components/icons/ReloadIcon.vue";
import BodyBuilder from "../components/BodyBuilder.vue";
export default {
  components: {
    Button,
    TrashIcon,
    PlusIcon,
    SaveIcon,
    OpenArrowIcon,
    Card,
    BodyBuilder,
    ReloadIcon,
    SettingsIcon,
    CodeIcon,
    DropdownSelect,
  },
  data() {
    return {
      config: {
        http: {
          defaults: {
            url: "",
            method: "GET",
            headers: [],
            queries: [],
            body: "",
          },
          history: { save_amount: 0 },
        },
      },
      previousConfig: {},
      open: {
        http: { open: true, headers: false, queries: false, body: false },
      },
    };
  },
  mounted() {
    this.loadConfig();
  },
  methods: {
    addHttpQuery() {
      console.log(this.config);
      this.config.http.defaults.queries.push({ name: "", value: "" });
    },
    deleteHttpQuery(index) {
      this.config.http.defaults.queries.splice(index, 1);
    },
    addHttpHeader() {
      this.config.http.defaults.headers.push({ name: "", value: "" });
    },
    deleteHttpHeader(index) {
      this.config.http.defaults.headers.splice(index, 1);
    },
    openConfig() {
      invoke("open_config");
    },
    async loadConfig() {
      let config = await invoke("get_config_values");
      console.log("got_config", config);
      this.config = JSON.parse(config);
      this.previousConfig = JSON.parse(JSON.stringify(this.config));
    },
    async saveConfig() {
      await invoke("save_config", {
        args: JSON.stringify(this.config, null, 2),
      });
      this.loadConfig();
    },
  },
};
</script>
