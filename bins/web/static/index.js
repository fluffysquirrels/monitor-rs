function indexMain() {
    console.log("indexMain");
    startWs();
}

let ws = null;

function startWs() {
    console.log("startWs");
    if (ws !== null) {
        console.error("startWs: ws !== null");
        return;
    }

    const url = (window.location.protocol === "https:" ? "wss://" : "ws://") +
        window.location.hostname + ":" + window.location.port + "/ws/";
    console.log("startWs connecting to:", url);
    ws = new WebSocket(url);
    ws.binaryType = "arraybuffer";
    ws.onclose = (evt) => {
        console.error("ws.onclose:", evt);
        ws = null;
    };
    ws.onerror = (evt) => {
        console.error("ws.onerror:", evt);
    };
    ws.onmessage = (evt) => {
        console.log("ws.onmessage:", evt);
        const arrayBuffer = evt.data;
        const view = new Uint8Array(arrayBuffer);
        const mws = protobuf.roots["default"].monitor_web_socket;
        var decode;
        try {
            decode = mws.ToClient.decode(view);
        } catch (e) {
            console.error("ws ToClient.decode error:", e);
            return;
        }
        console.log("ws ToClient.decode =", decode);
        if (decode.authResp) {
            console.log("ws authResp ok =", decode.authResp.ok);
        }
    };
    ws.onopen = (evt) => {
        console.log("ws.onopen:", evt);
        const mws = protobuf.roots["default"].monitor_web_socket;
        // TODO: Get the real key.
        const authReq = mws.AuthenticateRequest.create({ key: "foo" });
        const req = mws.ToServer.create({ authReq: authReq });
        const buf = mws.ToServer.encode(req).finish();
        console.log("auth buf:", buf);
        ws.send(buf);
    };
}

console.log("index.js loaded");
indexMain();
