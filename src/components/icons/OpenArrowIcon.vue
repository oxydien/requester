<style lang="scss" scoped>
svg {
  transition: all 200ms;
  transform: rotate(-90deg);
  &.open {
    transform: rotate(0);
  }
}
</style>

<template>
  <svg
    :class="{ open: open }"
    width="40"
    height="40"
    viewBox="0 0 40 40"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      d="M5.99998 13L19.4285 26.3028C19.8182 26.6889 20.4463 26.6889 20.836 26.3028L34 13.2621"
      stroke="white"
      stroke-width="7"
      stroke-linecap="round"
    />
  </svg>
</template>
<script>
export default {
  props: {
    open: {
      default: false,
      type: Boolean,
    },
  },
};
</script>
