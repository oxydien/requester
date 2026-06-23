<style scoped lang="scss">
.view-headers {
  display: block;
  margin: 0;
  padding: var(--gap-md);
  list-style: none;

  li {
    display: flex;

    strong {
      display: inline-block;
      min-width: 300px;
      margin-right: var(--gap-sm);
      text-align: right;
    }

    span {
      display: inline-block;
      text-wrap: wrap;
      word-wrap: break-word;
      max-width: calc(100vw - 300px - 120px);
    }
  }
}
</style>

<template>
  <ul class="view-headers">
    <li
        v-for="(header, index) in headers"
        :key="index"
    >
      <strong :style="`color: ${color}`">{{ header.name }}:</strong>
      <span>{{ header.value }}</span>
    </li>
  </ul>
</template>

<script>
export default {
  name: "HttpHeadersView",
  props: {
    color: null,
    headers: Array,
  }
}
</script>
