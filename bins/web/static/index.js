import * as Vue from "/static/third-party/vue.esm-browser.js";

function indexMain() {
    startWs();
    vueStart();
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
        setTimeout(() => {
            startWs();
        }, 5000 /* ms */);
    };
    ws.onerror = (evt) => {
        console.error("ws.onerror:", evt);
    };
    ws.onmessage = (evt) => {
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
        console.debug("ws ToClient.decode =", decode);
        if (decode.metricUpdate) {
            const m = decode.metricUpdate;
            console.debug("ws metricUpdate key =", m.metric.key);
            if (va) {
                const displayKey = m.metric.key.name + "@" + m.metric.key.fromHost.name;
                let displayValue = null;
                if (m.metric.latest.none) {
                    displayValue = m.metric.latest.ok ? "Ok": "Err";
                } else if (m.metric.latest.i64 !== undefined) {
                    displayValue = m.metric.latest.i64.toString();
                } else if (m.metric.latest.f64 !== undefined) {
                    displayValue = m.metric.latest.f64.toString();
                }

                const o = {
                    metricKey: displayKey,
                    ok: m.metric.latest.ok,
                    value: displayValue,
                };

                // TODO: Would be nice if this were O(1) somehow using a map.
                const existing = va.metrics.find(vm => vm.metricKey === displayKey);
                if (existing) {
                    existing.ok = o.ok;
                    existing.value = o.value;
                } else {
                    va.metrics.push(o);
                }
            }
        }
    };
    ws.onopen = (evt) => {
        console.log("ws.onopen:", evt);
        const mws = protobuf.roots["default"].monitor_web_socket;
        const subReq = mws.SubscribeToMetrics.create();
        const req = mws.ToServer.create({ subscribeToMetrics: subReq });
        const buf = mws.ToServer.encode(req).finish();
        ws.send(buf);
    };
}

let va = null;

function vueStart() {
    const app = {
        data() {
            return {
                metrics: [],
            };
        }
    };

    va = Vue.createApp(app).mount("#app");
    document.getElementById("app").style.display = "block";
    document.getElementById("pre-app").remove();
}

indexMain();
