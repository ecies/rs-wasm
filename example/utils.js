const hexes = Array.from({ length: 256 }, (_, i) =>
    i.toString(16).padStart(2, "0")
);

export function bytesToHex(bytes) {
    let hex = "";
    for (let i = 0; i < bytes.length; i++) {
        hex += hexes[bytes[i]];
    }
    return hex;
}
