import * as Vue from "/static/third-party/vue.esm-browser.js";
import * as webNotifier from "/static/web-notifier.js";
import * as util from "/static/util.js";
import * as webSocket from "/static/web-socket.js";

const monitor_web_socket = protobuf.roots["default"].monitor_web_socket;

let _webNotifier = null;
let vueApp = null;

function indexMain() {
    _webNotifier = webNotifier.create();
    vueStart();

    webSocket.setOnMetricsUpdate(handleMetricsUpdate);
    webSocket.setOnLogsUpdate(handleLogsUpdate);
    webSocket.setOnConnStatusChange((connStatus) => {
        console.info("connStatus = ", connStatus);
        vueApp.connStatus = connStatus;
    });
    webSocket.start();
}

function handleMetricsUpdate(metrics) {
    console.debug("handleMetricsUpdate len = ", metrics.length);
    if (vueApp === null) {
        return;
    }
    for (const metric of metrics) {
        const displayKey = protoKeyToDisplayKey(metric.key);
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

        const existing = findVueMetric(displayKey);
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

function handleLogsUpdate(logs) {
    console.debug("handleLogsUpdate len = ", logs.length);
    // TODO.
    for (const log of logs) {
        const displayKey = protoKeyToDisplayKey(log.key);
        const existing = findVueMetric(displayKey);
        if (!existing) {
            continue;
        }
        existing.log = log;
    }
}

// TODO: Would be nice if this were O(1) somehow using a map.
function findVueMetric(metricKey) {
    return vueApp.metrics.find(vm => vm.metricKey === metricKey);
}

function protoKeyToDisplayKey(protoKey) {
    return protoKey.name + "@" + protoKey.fromHost.name;
}

function vueStart() {
    const app = {
        data() {
            return {
                // Type of metrics is Array<
                //     metricKey: String,
                //     ok: Bool,
                //     value: String,
                //     log: monitor_core_types.Log,
                // >
                metrics: [],

                showAskNotificationPermission:
                    _webNotifier.permission() === "default",
                notificationStatus: _webNotifier.permission(),

                connStatus: {
                    state: webSocket.States.UNINITIALISED,
                },
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
