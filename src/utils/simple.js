export function copyToClipboard(text, el) {
  navigator.clipboard.writeText(text);
  if (el) {
    el[0].innerText = "Copied";
    setTimeout(() => {
      el[0].innerText = "Copy";
    }, 1500);
  }
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
