import {writeText} from "@tauri-apps/plugin-clipboard-manager";

export function copyToClipboard(text, el) {
  writeText(text).then(_ => {
    if (el) {
      el.innerText = "Copied";
      setTimeout(() => {
        el.innerText = "Copy";
      }, 1500);
    }
  });
}

export const deepCopy = (obj) => {
  if (obj === null || typeof obj !== "object") {
    return obj;
  }

  if (obj instanceof Array) {
    const copy = [];
    for (const item of obj) {
      copy.push(deepCopy(item));
    }
    return copy;
  }

  if (obj instanceof Object) {
    const copy = {};
    for (const key in obj) {
      if (obj.hasOwnProperty(key)) {
        copy[key] = deepCopy(obj[key]);
      }
    }
    return copy;
  }
};
