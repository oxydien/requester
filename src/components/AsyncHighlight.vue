<style scoped>
.async-hl {
  margin: 0;
  white-space: pre-wrap;
  word-break: break-word;
}

.async-hl--pending {
  opacity: 0.85;
}
</style>

<template>
  <pre v-if="highlighted === null" class="async-hl async-hl--pending"><code>{{ code }}</code></pre>
  <pre v-else class="async-hl hljs"><code v-html="highlighted"></code></pre>
</template>

<script>
import hljs from "highlight.js/lib/core";

export default {
  name: "AsyncHighlight",
  props: {
    code: {
      type: String,
      required: true,
    },
    language: {
      type: String,
      default: null,
    },
  },
  data() {
    return {
      highlighted: null,
    };
  },
  watch: {
    code: {
      immediate: true,
      handler() {
        this.scheduleHighlight();
      },
    },
    language() {
      this.scheduleHighlight();
    },
  },
  beforeUnmount() {
    this.cancelScheduled();
  },
  methods: {
    scheduleHighlight() {
      this.cancelScheduled();
      this.highlighted = null;

      const code = this.code;
      const language = this.language;

      const run = () => {
        this._pending = null;
        if (code !== this.code || language !== this.language) return;

        const result = language
            ? hljs.highlight(code, { language, ignoreIllegals: true })
            : hljs.highlightAuto(code);

        if (code === this.code && language === this.language) {
          this.highlighted = result.value;
        }
      };

      if (typeof requestIdleCallback === "function") {
        this._pending = requestIdleCallback(run, { timeout: 300 });
        this._cancelType = "idle";
      } else {
        this._pending = setTimeout(run, 0);
        this._cancelType = "timeout";
      }
    },
    cancelScheduled() {
      if (this._pending == null) return;
      if (this._cancelType === "idle" && typeof cancelIdleCallback === "function") {
        cancelIdleCallback(this._pending);
      } else {
        clearTimeout(this._pending);
      }
      this._pending = null;
    },
  },
};
</script>