<style scoped lang="scss">
.bar {
  position: sticky;
  display: flex;
  gap: var(--gap-sm);
  align-items: center;
  z-index: 2;
  padding-left: var(--_pl-current);
  background-color: var(--color-button-bg);
  color: var(--dark-color-contrast);
  cursor: pointer;
  box-sizing: border-box;
  border-bottom: var(--_base-border);
  transition: margin-top 100ms ease, padding-left 200ms ease;

  --_base-border: 2px solid #161819;

  --_h-big:    53px;
  --_h-normal: 43px;
  --_h-mid:    36px;
  --_h-tiny:   30px;

  --_h-small:  33px;

  --ot-normal: var(--_h-big);
  --ot-mid:    calc(var(--_h-big) + var(--_h-normal));
  --ot-tiny:   calc(var(--_h-big) + var(--_h-normal) + var(--_h-mid));

  --ot-small:  var(--_h-big);

  --_pl-normal: 24px;
  --_pl-mid: 36px;
  --_pl-tiny: 52px;

  --_pl-small: 24px;

  --_pl-current: 0;

  &::before {
    content: "";
    display: block;
    position: absolute;
    top: calc(50% - 1px);
    left: var(--gap-sm);
    width: calc(var(--_pl-current) - (8px + var(--gap-sm)));
    height: 2px;
    background-color: gray;
    border-radius: 3px;
  }
}

.big-bar {
  z-index: 5;
  top: 0;
  height: var(--_h-big);
  border-radius: var(--gap-md);
  border: none;
  padding-inline: var(--gap-sm);
  --_pl-current: var(--gap-sm);

  &.open {
    border-radius: var(--gap-md) var(--gap-md) 0 0;
    border: var(--_base-border);
  }

  &::before {
    display: none;
  }
}

.normal-bar {
  z-index: 4;
  top: var(--ot-normal);
  height: var(--_h-normal);
  --_pl-current: var(--_pl-normal);

  &.open {
    margin-top: var(--gap-md);
  }

  &::before {
    display: none;
  }
}

.mid-bar {
  z-index: 3;
  top: var(--ot-mid);
  height: var(--_h-mid);
  --_pl-current: var(--_pl-mid);
}

.small-bar {
  top: var(--ot-small);
  height: var(--_h-small);
  --_pl-current: var(--_pl-small);

  &::before {
    display: none;
  }
}

.tiny-bar {
  top: var(--ot-tiny);
  height: var(--_h-tiny);
  --_pl-current: var(--_pl-tiny);
}

.collapsable-body {
  background-color: #6F6F6F06;
  border-radius: 0 0 var(--radius-md) var(--radius-md);
  border-bottom: 1px solid #161819;
}

.big-body > *:last-child:not(.open) {
  border-radius: 0 0 var(--radius-md) var(--radius-md);
  overflow: hidden;
}

.normal-body {
  margin-bottom: var(--gap-md);
}
</style>

<template>
  <div
      :class="classes"
      @click="$emit('update:modelValue', !modelValue)"
  >
    <OpenArrowIcon :open="modelValue" />
    <slot name="header"></slot>
  </div>

  <div
      :class="classesBody"
      v-if="modelValue"
  >
    <slot name="content"></slot>
  </div>
</template>

<script>
import OpenArrowIcon from "./icons/OpenArrowIcon.vue";

export default {
  name: "CollapsableBar",
  components: {OpenArrowIcon},
  props: {
    modelValue: false,
    kind: {
      type: String,
      required: true,
      // big, normal, mid, small, tiny
    }
  },
  emits: ["update:modelValue"],
  computed: {
    classes() {
      return `bar ${this.kind}-bar ${this.modelValue ? "open" : ""}`;
    },
    classesBody() {
      return `collapsable-body ${this.kind}-body ${this.modelValue ? "open" : ""}`;
    }
  }
}
</script>
