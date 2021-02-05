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

    webSocket.setOnUpdate(handleUpdates);
    webSocket.setOnConnChange((connState) => console.info("connState = ", connState));
    webSocket.start();
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
