<style lang="scss" scoped>
.wrapper {
  display: flex;
  gap: var(--gap-sm);

  .page {
    width: calc(100vw - 3 * var(--gap-sm) - 60px);
    height: calc(100vh - (2 * var(--gap-sm)));
    margin: var(--gap-sm) 0 var(--gap-sm) 0;
    background-color: var(--color-raised-bg);
    border-radius: var(--radius-sm);
    overflow: auto;
  }
}
</style>

<template>
  <div class="wrapper">
    <NavHolder />
    <RouterView class="page" />
  </div>
</template>

<script>
import { defineComponent } from "vue";
import NavHolder from "./components/nav/NavHolder.vue";
import { invoke } from "@tauri-apps/api";

export default defineComponent({
  components: {
    NavHolder,
  },
  created() {
    invoke("app_loaded");
  },
});
</script>
