<style lang="scss" scoped>
.addr-holder {
  display: flex;
  align-items: stretch;
  gap: 10px;
  margin: 10px;

  input[type="text"] {
    width: 80%;
  }
}

.body-input {
  margin: 0 10px;
  width: calc(100% - 20px);
  max-width: calc(100% - 20px);
  height: 300px;
}

.error {
  border: 3px solid var(--color-red);
  color: var(--color-red);
  border-radius: var(--radius-md);
  margin: var(--gap-sm);
  padding: var(--gap-sm);
}
</style>

<template>
  <div id="NetIORequestPage">
    <section>
      <div class="section">
        <div class="addr-holder">
          <DropdownSelect
            name="selectType"
            id="NETIO_TYPE_DROPDOWN"
            style="max-width: 8ch; height: 40px"
            :options="['tcp', 'udp']"
            v-model="req_info.type"
          ></DropdownSelect>
          <input
            type="text"
            v-model="req_info.addr"
            placeholder="Host, Address, Domain..."
            @keydown.enter="sendRequest"
          />
          <input type="number" v-model="req_info.port" />
        </div>
      </div>
      <div class="section">
        <textarea
          class="body-input"
          placeholder="Insert text body..."
        ></textarea>
      </div>
      <Button
        color="secondary"
        style="margin: 10px auto; width: 80%"
        :disabled="status"
        @click="send_netoi_req"
      >
        <SendIcon />Send request
      </Button>
    </section>
    <div class="error">
      This page is currently a work in progress. Unfortunately, since there's no
      available method for testing this particular feature, it remains
      incomplete until someone contributes it on GitHub.
    </div>
    <section>
      <highlightjs
        autodetect
        :code="response ? response.trim() : 'REQUESTER-WARN: no data'"
      ></highlightjs>
    </section>
  </div>
</template>
<script>
import { invoke } from "@tauri-apps/api/tauri";
import {
  DropdownSelect,
  Button,
  TrashIcon,
  PlusIcon,
  SendIcon,
  ClipboardCopyIcon,
  Chips,
} from "omorphia";
export default {
  components: {
    DropdownSelect,
    Button,
    TrashIcon,
    PlusIcon,
    SendIcon,
    ClipboardCopyIcon,
    Chips,
  },
  data() {
    return {
      req_info: {
        type: "tcp",
        addr: "",
        port: 80,
        body: "",
      },
      response: "",
      status: false,
    };
  },
  methods: {
    async send_netoi_req() {
      let request = {
        protocol: this.req_info.type,
        host: this.req_info.addr,
        port: this.req_info.port,
        data: this.req_info.body,
      };
      this.status = this.req_info.type == "tcp";
      console.log("send_netio_req", request);
      let response;
      try {
        response = await invoke("send_netio_req", request);
        console.log("back_response", request, response);
        this.response = response;
        this.status = false;
      } catch (e) {
        this.response = e;
        console.log("bad_response", request, e);
        this.status = false;
      }
    },
  },
};
</script>
