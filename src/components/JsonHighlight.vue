<style lang="scss">
#jsonHighlightedCode {
  hyphens: auto;
  white-space: pre;
  overflow-wrap: normal;
  color: #dba1a1;
  .no-wrap {
    white-space: nowrap;
  }
}
</style>

<template>
  <div id="jsonHighlightedCode" v-html="highlightedJson"></div>
</template>

<script>
export default {
  props: {
    data: {
      required: true,
      type: String,
    },
  },
  computed: {
    highlightedJson() {
      const logText = this.data.replace(/</g, "&lt;").replace(/>/g, "&gt;");
      const regexes = [
        { pattern: /".*":/g, class: "#32daf5" },
        { pattern: /.*".*"/g, class: "#65fa5f" },
      ];
      let highlightedText = logText;
      regexes.forEach(({ pattern, class: cssClass }) => {
        highlightedText = highlightedText.replace(
          pattern,
          `<span style="color:${cssClass};">$&</span>`
        );
      });
      highlightedText = highlightedText
        .split("\n")
        .map((line) => {
          if (line.trim() === "") {
            return line; // Skip empty lines
          } else {
            return `<span class="no-wrap" style="white-space: pre">${line}</span>`;
          }
        })
        .join("\n");
      return highlightedText;
    },
  },
};
</script>
