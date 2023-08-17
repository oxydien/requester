<style lang="scss" scoped>
nav {
  --_width: 60px;
  display: flex;
  flex-flow: column nowrap;
  justify-content: space-between;
  gap: var(--gap-sm);
  background-color: var(--color-raised-bg);
  width: var(--_width);
  margin: var(--gap-sm) 0 0 var(--gap-sm);
  height: calc(100vh - (2 * var(--gap-sm)));
  padding: var(--gap-xs);
  border-radius: var(--radius-sm);

  .nav-item {
    width: 50px;
    height: 50px;
    text-align: center;
    padding: var(--gap-md);
    font-weight: 300;

    svg {
      margin: 0;
      max-width: unset;
      width: 30px;
      max-height: unset;
      height: 30px;
    }
  }
  .nav,
  .app-nav {
    display: flex;
    flex-flow: column nowrap;
    gap: var(--gap-sm);
  }
  .hr {
    display: block;
    height: 3px;
    width: 60%;
    background-color: var(--dark-color-contrast);
    margin: 0 auto;
    border-radius: 2px;
  }
}
</style>

<template>
  <nav>
    <div class="nav">
      <Button
        large
        :link="`/${page}/`"
        class="nav-item"
        v-for="(index, page) in config.pages"
        :color="
          $router.currentRoute.value.path == `/${page}/` ? 'secondary' : ''
        "
        :key="index"
        ><ReqTypeIcon :t="page.toLowerCase().trim()"
      /></Button>
    </div>
    <div class="app-nav">
      <div class="hr"></div>
      <Button
        iconOnly
        large
        class="nav-item"
        :color="$router.currentRoute.value.path == '/' ? 'secondary' : ''"
        link="/"
        ><ReqMngIcon
      /></Button>
      <Button
        iconOnly
        large
        class="nav-item"
        link="/settings/"
        :color="
          $router.currentRoute.value.path == '/settings/' ? 'secondary' : ''
        "
        ><SettingsIcon
      /></Button>
    </div>
  </nav>
</template>
<script>
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Button, SettingsIcon } from "omorphia";
import ReqMngIcon from "../icons/ReqMngIcon.vue";
import ReqTypeIcon from "../icons/ReqTypeIcon.vue";

export default defineComponent({
  components: {
    Button,
    SettingsIcon,
    ReqMngIcon,
    ReqTypeIcon,
  },
  data() {
    return {
      config: {
        pages: {},
      },
    };
  },
  mounted() {
    this.getConfig();
  },
  methods: {
    async getConfig() {
      this.config = JSON.parse(await invoke("get_config_values"));
    },
  },
});
</script>
