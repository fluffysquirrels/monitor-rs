//! Represents a WebSocket connection to the monitor-web server.

import * as util from "/static/util.js";

const monitor_web_socket = protobuf.roots["default"].monitor_web_socket;

export const States = {
    CONNECTING: "connecting",
    CONNECTED: "connected, pending data",
    ALIVE: "alive",
    CLOSED: "closed",
};

let ws = null;
let pingIntervalTimeout = null;
let onMetricsUpdate = (metrics) => {};
let onLogsUpdate = (logs) => {};
let onConnStatusChange = (connStatus) => {};
let state = States.CLOSED;

/// An array of { payload: Array<Number>, time: Date, timeoutId: TimeoutId }.
/// Each entry represents a ping we sent but haven't yet received a pong back.
let outstandingPings = [];

const PING_INTERVAL_MS             = 60 * 1000; // 60s
const PING_TIMEOUT_MS              = 15 * 1000; // 15s
const CONNECTION_RETRY_INTERVAL_MS = 5  * 1000; // 5s

/// For fault injection, ignore pongs.
const SKIP_PONGS = false;

export function setOnMetricsUpdate(cb) {
    onMetricsUpdate = cb;
}

export function setOnLogsUpdate(cb) {
    onLogsUpdate = cb;
}

export function setOnConnStatusChange(cb) {
    onConnStatusChange = cb;
}

export function start() {
    console.info("ws.start");
    if (ws !== null) {
        console.error("ws.start: ws !== null");
        return;
    }

    const url = (window.location.protocol === "https:" ? "wss://" : "ws://") +
        window.location.hostname + ":" + window.location.port + "/ws/";
    console.info("ws.start connecting to:", url);
    state = States.CONNECTING;
    callOnConnStatusChange();
    ws = new WebSocket(url);
    ws.binaryType = "arraybuffer";
    ws.onclose = (evt) => {
        console.error("ws.onclose:", evt);
        state = States.CLOSED;
        callOnConnStatusChange();
        shutdown();
        setRetry();
    };
    ws.onerror = (evt) => {
        console.error("ws.onerror:", evt);
    };
    ws.onmessage = (evt) => {
        if (state !== States.ALIVE) {
            state = States.ALIVE;
            callOnConnStatusChange();
        }
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
        handleMessage(decode);
    };
    ws.onopen = (evt) => {
        console.info("ws.onopen:", evt);
        state = States.CONNECTED;
        callOnConnStatusChange();
        startPings();

        const metricsSubReq = monitor_web_socket.SubscribeToMetrics.create();
        const metricsReq = monitor_web_socket.ToServer.create({
            subscribeToMetrics: metricsSubReq
        });
        send(metricsReq);

        const logsSubReq = monitor_web_socket.SubscribeToLogs.create();
        const logsReq = monitor_web_socket.ToServer.create({
            subscribeToLogs: logsSubReq
        });
        send(logsReq);
    };
}

/// Shutdown the WebSocket connection. Silent if the connection is already closed.
function shutdown(opts) {
    const { code } = opts || {};

    console.warn("ws.shutdown");

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

    state = States.CLOSED;
    callOnConnStatusChange();

    stopPings();

    // Unset onclose, onerror handlers which seem to fire (on Firefox) after we've
    // opened a new connection, which would end up calling shutdown() and
    // terminating the new connection.
    ws.onclose = null;
    ws.onerror = null;

    ws = null;
}

function setRetry() {
    setTimeout(() => {
        start();
    }, CONNECTION_RETRY_INTERVAL_MS);
}

function send(msg) {
    console.debug("ws.send: msg =", msg);
    const buf = monitor_web_socket.ToServer.encode(msg).finish();
    ws.send(buf);
}

/// Handle an incoming decoded message
function handleMessage(msg) {
    if (msg.metricsUpdate) {
        const m = msg.metricsUpdate;
        onMetricsUpdate(m.metrics);
    }
    else if (msg.logsUpdate) {
        const m = msg.logsUpdate;
        onLogsUpdate(m.logs);
    } else if (msg.pong) {
        if (SKIP_PONGS) {
            console.warn("Ignoring pong for debugging");
            return;
        }
        const pong = msg.pong;
        const ping =
            outstandingPings.find((x) => util.arraysEqual(x.payload, pong.payload));
        if (ping === undefined) {
            console.error("ws pong couldn't find ping payload =", pong.payload);
        } else {
            const duration = (new Date()) - ping.time;
            console.debug("ws Got pong, duration = " + duration + "ms");
            clearTimeout(ping.timeoutId);
            outstandingPings = outstandingPings.filter((x) => x !== ping);
        }
    } else {
        console.error("ws.handleMessage unknown message: ", msg);
    }
}

function callOnConnStatusChange() {
    onConnStatusChange({
        state,
    });
}

function startPings() {
    if (pingIntervalTimeout !== null) {
        console.error("ws.startPings pingIntervalTimeout !== null");
        return;
    }
    pingIntervalTimeout = setTimeout(sendPingLoop, PING_INTERVAL_MS);
}

function stopPings() {
    if (pingIntervalTimeout !== null) {
        clearTimeout(pingIntervalTimeout);
        pingIntervalTimeout = null;
    }
    outstandingPings = [];
}

function sendPingLoop() {
    const payload = util.randomBytes(16);
    const pingReq = monitor_web_socket.Ping.create({ payload });
    const req = monitor_web_socket.ToServer.create({ ping: pingReq });
    send(req);
    const outstandingPing = {
        payload,
        time: new Date(),
        timeoutId: setTimeout(() => outstandingPingTimeout(outstandingPing), PING_TIMEOUT_MS),
    };
    outstandingPings.push(outstandingPing);
    pingIntervalTimeout = setTimeout(sendPingLoop, PING_INTERVAL_MS);
}

/// A callback after waiting PING_TIMEOUT_MS after a ping request:
/// our ping has timed out and we assume the connection is dead.
function outstandingPingTimeout(outstandingPing) {
    console.warn("Outstanding WebSocket ping timeout, shutting down WebSocket.");
    shutdown();
    setRetry();
}
