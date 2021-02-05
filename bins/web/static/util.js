export function arraysEqual(a, b) {
    if (a === b) {
        return true;
    }
    if (a == null || b == null) {
        return false;
    }
    if (a.length !== b.length) {
        return false;
    }

    for (let idx = 0; idx < a.length; idx += 1) {
        if (a[idx] !== b[idx]) {
            return false;
        }
    }

    return true;
}

export function randomBytes(len) {
    const bytes = new Array(len);
    for (let idx = 0; idx < len; idx += 1) {
        const byte = Math.floor(Math.random() * 256);
        bytes[idx] = byte;
    }
    return bytes;
}
