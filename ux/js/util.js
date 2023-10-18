
const topicRegex = /topic\/([^\/]*)/

// get a topic id from the url, if it exists
export function getTopicId() {
    var matches = window.location.pathname.match(topicRegex);

    if (matches === null) {
        return null;
    }

    return matches[1];
}

// fetch the user's uuid from the window
export function getUuid() {
    return window.data.uuid;
}