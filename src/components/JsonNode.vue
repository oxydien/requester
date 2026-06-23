<style lang="scss" scoped>
.jt-node {
  font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
  font-size: 13px;
  line-height: 1.5;
}

.jt-line {
  white-space: pre;
  display: block;

  &--clickable {
    cursor: pointer;

    &:hover {
      background: rgba(255, 255, 255, 0.04);
    }
  }
}

.jt-toggle {
  display: inline-block;
  width: 14px;
  color: #c5b9b9;
  font-size: 10px;
}

.jt-key {
  color: #5de0d8;
}

.jt-index {
  color: #47bbeb;
}

.jt-punct {
  color: #418cec;
}

.jt-string {
  color: #e375e7;
}

.jt-number {
  color: #75ef42;
}

.jt-boolean {
  color: #f57f1c;
}

.jt-null {
  color: #de7575;
  font-style: italic;
}

.jt-error {
  color: #de2f2f;
}

.jt-preview {
  color: #999;
  font-style: italic;
  margin: 0 2px;
}

.jt-chunk-label {
  color: #bfb9b9;
  font-style: italic;
}

.jt-children {
  display: block;
}
</style>

<template>
  <div class="jt-node">
    <div
        class="jt-line"
        :class="{ 'jt-line--clickable': isExpandable }"
        :style="{ paddingLeft: depth * 14 + 'px' }"
        @click="isExpandable && toggleCollapsed()"
    >
      <span class="jt-toggle">
        <template v-if="isExpandable">{{ collapsed ? '\u25B6' : '\u25BC' }}</template>
      </span>

      <span v-if="keyName !== null && !isArrayItem" class="jt-key">"{{ keyName }}"</span>
      <span v-else-if="keyName !== null && isArrayItem" class="jt-index">{{ keyName }}</span>
      <span v-if="keyName !== null" class="jt-punct">:&nbsp;</span>

      <template v-if="parseError">
        <span class="jt-error">{{ parseError }}</span>
      </template>

      <template v-else-if="!isExpandable">
        <span :class="valueClass">{{ displayValue }}</span>
        <span v-if="!isLast" class="jt-punct">,</span>
      </template>

      <template v-else>
        <span class="jt-punct">{{ openBracket }}</span>
        <template v-if="collapsed">
          <span class="jt-preview">{{ previewText }}</span>
          <span class="jt-punct">{{ closeBracket }}</span>
          <span v-if="!isLast" class="jt-punct">,</span>
        </template>
      </template>
    </div>

    <div v-if="isExpandable && !collapsed" class="jt-children">
      <template v-if="chunkGroups">
        <div v-for="(grp, gi) in chunkGroups" :key="'chunk-' + gi" class="jt-chunk">
          <div
              class="jt-line jt-line--clickable jt-chunk-header"
              :style="{ paddingLeft: (depth + 1) * 14 + 'px' }"
              @click="toggleChunk(gi)"
          >
            <span class="jt-toggle">{{ chunkCollapsed[gi] === false ? '\u25BC' : '\u25B6' }}</span>
            <span class="jt-chunk-label">items {{ grp.start }}…{{ grp.end }} ({{ grp.end - grp.start + 1 }})</span>
          </div>
          <div v-if="chunkCollapsed[gi] === false" class="jt-children">
            <json-node
                v-for="i in rangeArray(grp.start, grp.end)"
                :key="i"
                :value="actualValue[i]"
                :key-name="i"
                :is-array-item="true"
                :depth="depth + 1"
                :is-last="i === actualValue.length - 1"
                :max-depth="maxDepth"
                :chunk-size="chunkSize"
            />
          </div>
        </div>
      </template>

      <template v-else>
        <json-node
            v-for="(item, idx) in entries"
            :key="idx"
            :value="item.value"
            :key-name="item.key"
            :is-array-item="valueType === 'array'"
            :depth="depth + 1"
            :is-last="idx === entries.length - 1"
            :max-depth="maxDepth"
            :chunk-size="chunkSize"
        />
      </template>

      <div class="jt-line jt-close" :style="{ paddingLeft: depth * 14 + 'px' }">
        <span class="jt-toggle"></span>
        <span class="jt-punct">{{ closeBracket }}</span>
        <span v-if="!isLast" class="jt-punct">,</span>
      </div>
    </div>
  </div>
</template>

<script>
import jsonBigIntBuilder from "json-bigint";

const JSONbig = jsonBigIntBuilder({useNativeBigInt: true})

export default {
  name: "JsonNode",
  props: {
    // Root usage: pass the raw JSON string here.
    data: {
      type: String,
      default: null,
    },
    // Internal recursive usage: pass the already-parsed value.
    value: {
      default: undefined,
    },
    keyName: {
      default: null,
    },
    isArrayItem: {
      type: Boolean,
      default: false,
    },
    depth: {
      type: Number,
      default: 0,
    },
    isLast: {
      type: Boolean,
      default: true,
    },
    maxDepth: {
      type: Number,
      default: 1,
    },
    chunkSize: {
      type: Number,
      default: 100,
    },
  },
  inject: {
    injectedSignal: { default: null },
  },
  data() {
    const parsed = this.resolveValue();
    const expandable = this.checkExpandable(parsed.value);

    const lastCommand = this.injectedSignal ? this.injectedSignal.command : null;
    let initialCollapsed;
    if (lastCommand === "collapse") {
      initialCollapsed = expandable;
    } else if (lastCommand === "expand") {
      initialCollapsed = false;
    } else {
      initialCollapsed = expandable && this.depth >= this.maxDepth;
    }

    const chunkCollapsed = {};
    if (expandable && this.getType(parsed.value) === "array" && parsed.value.length > this.chunkSize) {
      const count = Math.ceil(parsed.value.length / this.chunkSize);
      for (let i = 0; i < count; i++) {
        chunkCollapsed[i] = lastCommand !== "expand";
      }
    }

    return {
      parseError: parsed.error,
      collapsed: initialCollapsed,
      chunkCollapsed,
      ownSignal: { command: null, version: 0 },
    };
  },
  provide() {
    return {
      injectedSignal: this.injectedSignal || this.ownSignal,
    };
  },
  watch: {
    "collapseSignal.version"() {
      this.setAllCollapsed(this.collapseSignal.command === "collapse");
    },
  },
  computed: {
    collapseSignal() {
      return this.injectedSignal || this.ownSignal;
    },
    actualValue() {
      return this.resolveValue().value;
    },
    valueType() {
      return this.getType(this.actualValue);
    },
    isExpandable() {
      if (this.parseError) return false;
      return this.checkExpandable(this.actualValue);
    },
    entries() {
      if (this.valueType === "array") {
        return this.actualValue.map((v, i) => ({ key: i, value: v }));
      }
      if (this.valueType === "object") {
        return Object.keys(this.actualValue).map((k) => ({ key: k, value: this.actualValue[k] }));
      }
      return [];
    },
    chunkGroups() {
      if (this.valueType !== "array") return null;
      const len = this.actualValue.length;
      if (len <= this.chunkSize) return null;
      const groups = [];
      for (let start = 0; start < len; start += this.chunkSize) {
        groups.push({ start, end: Math.min(start + this.chunkSize, len) - 1 });
      }
      return groups;
    },
    openBracket() {
      return this.valueType === "array" ? "[" : "{";
    },
    closeBracket() {
      return this.valueType === "array" ? "]" : "}";
    },
    displayValue() {
      return this.formatPrimitive(this.actualValue);
    },
    valueClass() {
      return "jt-" + this.valueType;
    },
    previewText() {
      const items = this.valueType === "array" ? this.actualValue : Object.keys(this.actualValue);
      const count = items.length;
      if (count === 0) return "";
      const pieces = [];
      const limit = 3;
      for (let i = 0; i < Math.min(limit, count); i++) {
        if (this.valueType === "array") {
          pieces.push(this.previewFragment(this.actualValue[i]));
        } else {
          const k = items[i];
          pieces.push(k + ": " + this.previewFragment(this.actualValue[k]));
        }
      }
      const more = count > limit ? ", \u2026" : "";
      return pieces.join(", ") + more;
    },
  },
  methods: {
    resolveValue() {
      if (this.depth === 0 && this.data !== null) {
        try {
          return { value: JSONbig.parse(this.data), error: null };
        } catch (e) {
          return { value: null, error: "Invalid JSON: " + e.message };
        }
      }
      return { value: this.value, error: null };
    },
    getType(val) {
      if (val === null) return "null";
      if (Array.isArray(val)) return "array";
      return typeof val;
    },
    checkExpandable(val) {
      const type = this.getType(val);
      if (type === "array") return val.length > 0;
      if (type === "object") return Object.keys(val).length > 0;
      return false;
    },
    formatPrimitive(val) {
      const type = this.getType(val);
      if (type === "string") return '"' + val.replaceAll("\n", "\\n") + '"';
      if (type === "null") return "null";
      if (type === "undefined") return "undefined";
      if (type === "object" && Object.keys(val).length === 0) return "{}"
      if (type === "array" && val.length === 0) return "[]"
      return String(val);
    },
    previewFragment(val) {
      const type = this.getType(val);
      if (type === "array") return "Array(" + val.length + ")";
      if (type === "object") return "{\u2026}";
      return this.formatPrimitive(val);
    },
    toggleCollapsed() {
      this.collapsed = !this.collapsed;
    },
    toggleChunk(i) {
      this.chunkCollapsed[i] = this.chunkCollapsed[i] === false;
    },
    rangeArray(start, end) {
      const arr = [];
      for (let i = start; i <= end; i++) arr.push(i);
      return arr;
    },
    setAllCollapsed(collapseFlag) {
      if (this.isExpandable) this.collapsed = collapseFlag;
      if (this.chunkGroups) {
        this.chunkGroups.forEach((_, i) => {
          this.chunkCollapsed[i] = !!collapseFlag;
        });
      }
    },
    collapseAll() {
      this.setAllCollapsed(true);
      this.collapseSignal.command = "collapse";
      this.collapseSignal.version++;
    },
    expandAll() {
      this.setAllCollapsed(false);
      this.collapseSignal.command = "expand";
      this.collapseSignal.version++;
    },
  },
};
</script>