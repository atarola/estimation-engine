import { register } from "./actions/register";

let timeoutHandle = 0;

export let socket;

// create a websocket for the application
export function createWebSocket(id, name, dispatch) {
    let proto = (window.location.protocol == "https:") ? "wss:" : "ws:";    

    // if we haven't seen a ping for a while, try to reconnect
    let resetTimeout = () => {
        clearTimeout(timeoutHandle);
        timeoutHandle = setTimeout(() => {
            dispatch(register({ name: name }));
        }, 30000);
    } 

    socket = new WebSocket(`${proto}//${window.location.host}/ws/${id}`);
    socket.onmessage = (event) => {
        dispatch(JSON.parse(event.data));
        resetTimeout();
    };
}
