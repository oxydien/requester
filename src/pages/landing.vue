<style lang="scss" scoped>
.create-request {
  display: flex;
  flex-flow: column;
  min-width: 220px;
  svg {
    width: 80px;
    height: 80px;
  }
}
.flex-grid {
  display: flex;
  gap: var(--gap-sm);
  flex-flow: row wrap;
  justify-content: center;
  width: 60%;
  background-color: var(--color-super-raised-bg);
  padding: var(--gap-sm);
  border-radius: var(--radius-sm);
  margin: 0 auto;
}
[class~="e"] {
  position: absolute;
}
[class~="e"] {
  bottom: 15pt;
}
[class~="e"] {
  right: 15pt;
}
[id~="e"] {
  text-decoration: none;
}
.scroll-top-button {
  position: absolute;
  bottom: 45px;
  right: 18px;
}
.changelog-box {
  display: block;
  position: relative;
  width: 605px;
  left: 50%;
  transform: translateX(-50%);
  margin: 30px 0;
  padding: 10px;
  text-align: center;
  border: 1px solid var(--dark-color-base);
  &.major {
    border: 1px solid var(--dark-color-contrast);
    color: var(--dark-color-contrast);
  }
  &.current {
    border: 1px solid var(--color-requester-dark);
    span {
      color: var(--color-requester);
    }
  }
  border-radius: 15px;
  span {
    position: absolute;
    top: -10px;
    left: 30px;
    background-color: var(--color-raised-bg);
    padding: 0 5px;
    font-weight: 600;
  }
}
h3 {
  position: sticky;
  top: 0;
  display: block;
  text-align: center;
  margin: 0;
  padding: 10px 0;
  z-index: 50;
  background-color: var(--color-raised-bg);
}
</style>

<template>
  <div id="landingPage" ref="landingPage">
    <Logo style="margin-top: 50px" />

    <div class="flex-grid">
      <Button
        class="create-request"
        v-for="(page, index) in config.pages"
        :key="index"
        :link="index + '/'"
      >
        <ReqTypeIcon :t="index" /> {{ page }}</Button
      >
    </div>

    <h3>Changelog</h3>
    <div class="changelog-wrapper">
      <div
        v-for="change in changelog"
        :key="change.title"
        class="changelog-box"
        :class="{
          major: change.major,
          current: change.version === config.version,
        }"
      >
        <span>{{ change.title }}</span>
        <p v-html="change.message"></p>
      </div>
    </div>
    <Button
      v-if="scrollPosition > 300"
      class="scroll-top-button"
      iconOnly
      @click="scrollToTop"
    >
      <UpIcon />
    </Button>

    <div class="e">
      &#77;&#97;&#100;&#101;&#32;&#98;&#121;&#32;
      <a id="e">&#111;&#120;&#121;&#100;&#105;&#101;&#110;</a>
    </div>
  </div>
</template>
<script>
// This page was made in rush
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Button } from "omorphia";
import ReqTypeIcon from "../components/icons/ReqTypeIcon.vue";
import UpIcon from "../components/icons/UpIcon.vue";
import Logo from "../components/icons/Logo.vue";
import { changelog } from "../utils/changelog";

export default defineComponent({
  components: {
    Button,
    ReqTypeIcon,
    Logo,
    UpIcon,
  },
  data() {
    return {
      config: { pages: [], version: "" },
      scrollPosition: 0,
    };
  },
  mounted() {
    this.getConfig();
    eval(
      "\x64\x6F\x63\x75\x6D\x65\x6E\x74\x2E\x67\x65\x74\x45\x6C\x65\x6D\x65\x6E\x74\x42\x79\x49\x64\x28\x22\x65\x22\x29\x2E\x68\x72\x65\x66\x20\x3D\x20\x22\x68\x74\x74\x70\x73\x3A\x2F\x2F\x67\x69\x74\x68\x75\x62\x2E\x63\x6F\x6D\x2F\x6F\x78\x79\x64\x69\x65\x6E\x22\x3B\x64\x6F\x63\x75\x6D\x65\x6E\x74\x2E\x67\x65\x74\x45\x6C\x65\x6D\x65\x6E\x74\x42\x79\x49\x64\x28\x22\x65\x22\x29\x2E\x74\x61\x72\x67\x65\x74\x20\x3D\x20\x22\x5F\x62\x6C\x61\x6E\x6B\x22\x3B"
    );
    this.$refs["landingPage"].addEventListener("scroll", this.handleScroll);
  },
  beforeUnmount() {
    this.$refs["landingPage"].removeEventListener("scroll", this.handleScroll);
  },
  methods: {
    async getConfig() {
      this.config = JSON.parse(await invoke("get_config_values"));
    },
    scrollToTop() {
      this.$refs["landingPage"].scrollTo({
        top: 0,
        behavior: "smooth",
      });
    },
    handleScroll() {
      this.scrollPosition = this.$refs["landingPage"].scrollTop;
    },
  },
  computed: {
    changelog() {
      const sortedChangelog = changelog.slice().sort((a, b) => {
        const versionA = a.version.split(".").map(Number);
        const versionB = b.version.split(".").map(Number);

        for (let i = 0; i < Math.min(versionA.length, versionB.length); i++) {
          if (versionA[i] !== versionB[i]) {
            return versionB[i] - versionA[i];
          }
        }
        return versionB.length - versionA.length;
      });

      return sortedChangelog;
    },
  },
});
</script>
