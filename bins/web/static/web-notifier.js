const storage = window.localStorage;
const DISABLED_KEY = "notifier__disabled";

export function create() {
    if (!("Notification" in window)) {
        console.error("This browser does not support desktop notification");
    }

    return new Notifier();
}

class Notifier {
    permission() {
        if (!Notification) {
            return "not supported";
        }

        if (this.isDisabled()) {
            return "disabled";
        }

        return Notification.permission;
    }

    isDisabled() {
        return storage.getItem(DISABLED_KEY) === "1";
    }

    setDisabled(val) {
        return storage.setItem(DISABLED_KEY, val ? "1" : "0");
    }

    disable() {
        this.setDisabled(true);
    }

    requestPermission() {
        if (!Notification) {
            return Promise.resolve({ permission: "not supported" });
        }

        this.setDisabled(false);

        const p = new Promise((resolveP, rejectP) => {
            Notification.requestPermission().then((perm) => {
                resolveP({ permission: perm });
            });
        });
        return p;
    }

    exampleNotification() {
        this.notify("Notifications example", {});
    }

    notify(title, opts) {
        if (this.permission() !== "granted") {
            return;
        }

        const n = new Notification(title, opts);
        if (opts.timeoutMs === undefined || opts.timeoutMs === null) {
            opts.timeoutMs = 5000;
        }
        if (opts.timeoutMs > 0) {
            setTimeout(() => n.close(), opts.timeoutMs);
        }
    }
}
