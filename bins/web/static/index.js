import * as Vue from "/static/third-party/vue.esm-browser.js";
import * as webNotifier from "/static/web-notifier.js";
import * as util from "/static/util.js";

const monitor_web_socket = protobuf.roots["default"].monitor_web_socket;

function indexMain() {
    _webNotifier = webNotifier.create();
    vueStart();
    wsStart();
}

let _webNotifier = null;

let ws = null;
let wsPingIntervalTimeout = null;

const PING_INTERVAL_MS             = 60 * 1000; // 60s
const PING_TIMEOUT_MS              = 15 * 1000; // 15s

const CONNECTION_RETRY_INTERVAL_MS = 5  * 1000; // 5s

function wsStart() {
    console.info("wsStart");
    if (ws !== null) {
        console.error("wsStart: ws !== null");
        return;
    }

    const url = (window.location.protocol === "https:" ? "wss://" : "ws://") +
        window.location.hostname + ":" + window.location.port + "/ws/";
    console.info("wsStart connecting to:", url);
    ws = new WebSocket(url);
    ws.binaryType = "arraybuffer";
    ws.onclose = (evt) => {
        console.error("ws.onclose:", evt);
        wsShutdown();
        wsSetRetry();
    };
    ws.onerror = (evt) => {
        console.error("ws.onerror:", evt);
    };
    ws.onmessage = (evt) => {
        const arrayBuffer = evt.data;
        const view = new Uint8Array(arrayBuffer);
        var decode;
        try {
            decode = monitor_web_socket.ToClient.decode(view);
        } catch (e) {
            console.error("ws ToClient.decode error:", e);
            return;
        }
        console.debug("ws.onmessage ToClient.decode =", decode);
        wsHandleMessage(decode);
    };
    ws.onopen = (evt) => {
        console.log("ws.onopen:", evt);
        wsStartPings();
        const subReq = monitor_web_socket.SubscribeToMetrics.create();
        const req = monitor_web_socket.ToServer.create({ subscribeToMetrics: subReq });
        wsSend(req);
    };
}

/// Shutdown the WebSocket connection. Silent if the connection is already closed.
function wsShutdown(opts) {
    const { code } = opts || {};

    console.warn("wsShutdown");

    if (ws === null) {
        return;
    }

    if (ws.readyState !== WebSocket.CLOSING &&
            ws.readyState !== WebSocket.CLOSED) {
        if (code) {
            ws.close(code);
        } else {
            ws.close();
        }
    }

    wsStopPings();

    ws = null;
}

function wsSetRetry() {
    setTimeout(() => {
        wsStart();
    }, CONNECTION_RETRY_INTERVAL_MS);
}

function wsSend(msg) {
    console.debug("wsSend: msg =", msg);
    const buf = monitor_web_socket.ToServer.encode(msg).finish();
    ws.send(buf);
}

/// Handle an incoming decoded message
function wsHandleMessage(msg) {
    if (msg.metricsUpdate) {
        const m = msg.metricsUpdate;
        handleUpdates(m.metrics);
    } else if (msg.pong) {
        const pong = msg.pong;
        const ping =
            wsOutstandingPings.find((x) => util.arraysEqual(x.payload, pong.payload));
        if (ping === undefined) {
            console.error("ws pong couldn't find ping payload =", pong.payload);
        } else {
            const duration = (new Date()) - ping.time;
            console.debug("ws Got pong, duration = " + duration + "ms");
            clearTimeout(ping.timeoutId);
            wsOutstandingPings = wsOutstandingPings.filter((x) => x !== ping);
        }
    } else {
        console.error("wsHandleMessage unknown message: ", msg);
    }
}

function handleUpdates(metrics) {
    console.debug("handleUpdates len = ", metrics.length);
    if (vueApp === null) {
        return;
    }
    for (const metric of metrics) {
        const displayKey = metric.key.name + "@" + metric.key.fromHost.name;
        let displayValue = null;
        if (metric.latest.none) {
            displayValue = metric.latest.ok ? "Ok": "Err";
        } else if (metric.latest.i64 !== undefined) {
            displayValue = metric.latest.i64.toString();
        } else if (metric.latest.f64 !== undefined) {
            displayValue = metric.latest.f64.toString();
        }

        const o = {
            metricKey: displayKey,
            ok: metric.latest.ok,
            value: displayValue,
        };

        // TODO: Would be nice if this were O(1) somehow using a map.
        const existing = vueApp.metrics.find(vm => vm.metricKey === displayKey);
        if (existing) {
            existing.ok = o.ok;
            existing.value = o.value;
        } else {
            vueApp.metrics.push(o);
        }

        if (!metric.latest.ok) {
            _webNotifier.notify(`Error for metric '${displayKey}'`, {
                body: `Value: ${displayValue}`,
            });
        }
    }
}

function wsStartPings() {
    if (wsPingIntervalTimeout !== null) {
        console.error("wsStartPing wsPingIntervalTimeout !== null");
        return;
    }
    wsPingIntervalTimeout = setTimeout(wsSendPingLoop, PING_INTERVAL_MS);
}

function wsStopPings() {
    if (wsPingIntervalTimeout !== null) {
        clearTimeout(wsPingIntervalTimeout);
        wsPingIntervalTimeout = null;
    }
    wsOutstandingPings = [];
}

/// An array of { payload: Array<Number>, time: Date, timeoutId: TimeoutId }.
/// Each entry represents a ping we sent but haven't yet received a pong back.
let wsOutstandingPings = [];

function wsSendPingLoop() {
    const payload = util.randomBytes(16);
    const pingReq = monitor_web_socket.Ping.create({ payload });
    const req = monitor_web_socket.ToServer.create({ ping: pingReq });
    wsSend(req);
    const outstandingPing = {
        payload,
        time: new Date(),
        timeoutId: setTimeout(() => wsOutstandingPingTimeout(outstandingPing), PING_TIMEOUT_MS),
    };
    wsOutstandingPings.push(outstandingPing);
    wsPingIntervalTimeout = setTimeout(wsSendPingLoop, PING_INTERVAL_MS);
}

/// A callback after waiting PING_TIMEOUT_MS after a ping request:
/// our ping has timed out and we assume the connection is dead.
function wsOutstandingPingTimeout(outstandingPing) {
    console.warn("Outstanding WebSocket ping timeout, shutting down WebSocket.");
    wsShutdown();
    wsSetRetry();
}

let vueApp = null;

function vueStart() {
    const app = {
        data() {
            return {
                metrics: [],

                showAskNotificationPermission:
                    _webNotifier.permission() === "default",
                notificationStatus: _webNotifier.permission(),
            };
        },
        computed: {
            metricsNumOk() {
                return this.metrics.filter(m => m.ok).length;
            },
            metricsNumErr() {
                return this.metrics.filter(m => !m.ok).length;
            },
        },
        methods: {
            notificationsAllow(evt) {
                _webNotifier.requestPermission().then((evt) => {
                    vueApp.notificationStatus = evt.permission;
                });
                vueApp.notificationStatus = "requested";
                vueApp.showAskNotificationPermission = false;
            },
            notificationsDisable(evt) {
                _webNotifier.disable();
                vueApp.notificationStatus = "disabled";
                vueApp.showAskNotificationPermission = false;
            },
            notificationsTest() {
                _webNotifier.exampleNotification();
            },
        },
    };

    vueApp = Vue.createApp(app).mount("#app");
    document.getElementById("app").style.display = "block";
    document.getElementById("pre-app").remove();
}

indexMain();
