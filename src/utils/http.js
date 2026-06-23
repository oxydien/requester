export const defaultOpenResponseTabs = {
  open: true,
  request: { all: true, headers: false, body: false },
  response: { all: true, headers: false, body: true, bodyType: "raw" },
};
export const defaultClosedResponseTabs = {
  open: false,
  request: { all: true, headers: false, body: true },
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
    const decodedValue = value ? decodeURIComponent(value) : "";
    queries.push({ name: decodedName, value: decodedValue });
  });

  return queries;
}

export function validateUrl(url) {
  const urlPattern =
    "^(?:(?:(?:https?|ftp):)?\\/\\/)(?:\\S+(?::\\S*)?@)?(?:(?!(?:10|127)(?:\\.\\d{1,3}){3})(?!(?:169\\.254|192\\.168)(?:\\.\\d{1,3}){2})(?!172\\.(?:1[6-9]|2\\d|3[0-1])(?:\\.\\d{1,3}){2})(?:[1-9]\\d?|1\\d\\d|2[01]\\d|22[0-3])(?:\\.(?:1?\\d{1,2}|2[0-4]\\d|25[0-5])){2}(?:\\.(?:[1-9]\\d?|1\\d\\d|2[0-4]\\d|25[0-4]))|(?:(?:[a-z0-9\u00a1-\uffff][a-z0-9\u00a1-\uffff_-]{0,62})?[a-z0-9\u00a1-\uffff]\\.)+(?:[a-z\u00a1-\uffff]{2,}\\.?))(?::\\d{2,5})?(?:[/?#]\\S*)?$";
  const regex = new RegExp(urlPattern);
  return regex.test(url);
}
