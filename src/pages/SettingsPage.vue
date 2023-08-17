<style lang="scss" scoped>
#settingsPage {
  padding: 1rem;
  h1 {
    margin: 10px 0;
  }
}
.card {
  background-color: var(--color-bg);
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
    <h1>
      Settings
      <Button
        style="display: inline; float: right; font-size: 1.2rem"
        color="highlight"
        :disabled="JSON.stringify(config) == JSON.stringify(previousConfig)"
        @click="saveConfig"
        ><SaveIcon />Save</Button
      >
    </h1>
    <Card class="card">
      <h3>Default Http Request Queries</h3>
      <div class="section queries">
        <div
          v-for="(query, index) in config.defaults.queries"
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
          <Button @click="deleteQuery(index)" iconOnly>
            <TrashIcon />
          </Button>
        </div>
        <Button @click="addQuery" iconOnly class="add-query">
          <PlusIcon />
        </Button>
      </div>
    </Card>
    <Card class="card">
      <h3>Default Http Request Headers</h3>
      <div class="section headers">
        <div
          v-for="(header, index) in config.defaults.headers"
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
          <Button @click="deleteHeader(index)" iconOnly>
            <TrashIcon />
          </Button>
        </div>
        <Button @click="addHeader" iconOnly class="add-header">
          <PlusIcon />
        </Button>
      </div>
    </Card>
    <Card class="card">
      <h3>Default Http Request Body</h3>
      <textarea
        name="reqBody"
        id="reqBody"
        style="
          min-height: 200px;
          width: 100%;
          max-width: calc(100% - 10px);
          margin: 10px 5px;
        "
        placeholder="Default http request body..."
        v-model="config.defaults.body"
      ></textarea>
    </Card>
  </div>
</template>
<script>
import { invoke } from "@tauri-apps/api/tauri";
import { Button, TrashIcon, PlusIcon, Card, SaveIcon } from "omorphia";
import OpenArrowIcon from "../components/icons/OpenArrowIcon.vue";
export default {
  components: {
    Button,
    TrashIcon,
    PlusIcon,
    SaveIcon,
    OpenArrowIcon,
    Card,
  },
  data() {
    return {
      config: { defaults: { headers: [] } },
      previousConfig: {},
    };
  },
  mounted() {
    this.loadConfig();
  },
  methods: {
    addQuery() {
      console.log(this.config);
      this.config.defaults.queries.push({ name: "", value: "" });
    },
    deleteQuery(index) {
      this.config.defaults.queries.splice(index, 1);
    },
    addHeader() {
      this.config.defaults.headers.push({ name: "", value: "" });
    },
    deleteHeader(index) {
      this.config.defaults.headers.splice(index, 1);
    },
    async loadConfig() {
      let config = await invoke("get_config_values");
      console.log(config);
      this.config = JSON.parse(config);
      this.previousConfig = JSON.parse(JSON.stringify(this.config));
    },
    async saveConfig() {
      await invoke("save_config", { args: JSON.stringify(this.config) });
      this.loadConfig();
    },
  },
};
</script>
