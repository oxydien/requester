<template>
  <div
    style="
      background-color: black;
      display: flex;
      flex-flow: column nowrap;
      justify-content: center;
      align-items: center;
      height: 100vh;
      width: 100vw;
    "
  >
    <img src="../../src/assets/img/logo.svg" style="width: 60vw" />
    <span
      style="
        font-size: 3.8rem;
        color: white;
        font-family: Arial, Helvetica, sans-serif;
      "
      >{{ message || "Loading..." }}</span
    >
  </div>
</template>
<script>
export default {
  props: {
    message: {
      type: String,
      default: null,
    },
  },
};
</script>
