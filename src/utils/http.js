export const defaultOpenResponseTabs = {
  open: true,
  request: { all: false, headers: false, body: true },
  response: { all: true, headers: false, body: true, bodyType: "raw" },
};
export const defaultClosedResponseTabs = {
  open: false,
  request: { all: false, headers: false, body: true },
  response: { all: true, headers: false, body: true, bodyType: "raw" },
};

export function findContentTypeInHeaders(headers) {
  if (!headers) {
    return "text";
  }

  const contentTypeHeader = headers.find(
    (header) => header.name.toLowerCase() === "content-type"
  );

  if (contentTypeHeader) {
    const contentTypeValue = contentTypeHeader.value.toLowerCase();
    if (contentTypeValue.includes("html") || contentTypeValue.includes("htm")) {
      return "html";
    } else if (contentTypeValue.includes("json")) {
      return "json";
    } else if (
      contentTypeValue.includes("image") ||
      contentTypeValue.includes("img")
    ) {
      return "image";
    } else {
      return "text";
    }
  }

  return "text";
}

export function parseUrlToQueries(url) {
  const queryIndex = url.indexOf("?");
  if (queryIndex === -1) {
    return [];
  }

  const queryString = url.slice(queryIndex + 1);
  const queryPairs = queryString.split("&");
  const queries = [];

  queryPairs.forEach((pair) => {
    const [name, value] = pair.split("=");
    const decodedName = decodeURIComponent(name);
    const decodedValue = decodeURIComponent(value);
    queries.push({ name: decodedName, value: decodedValue });
  });

  return queries;
}

export function validateUrl(url) {
  const urlPattern =
    "^(https?:\\/\\/|~\\/|\\/)?([\\w]+:\\w+@)?([a-zA-Z]{1}([\\w-]+\\.)+([\\w]{2,5}))(:[\\d]{1,5})?((\\/\\w+\\/)+|\\/)?(\\w+\\.[\\w]{3,4})?((\\?\\w+=\\w+)?(&\\w+=\\w+)*)?(#[\\w .-]*)?$";
  const regex = new RegExp(urlPattern);
  return regex.test(url);
}
