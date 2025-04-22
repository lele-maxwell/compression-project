// RLE compression
function compress(data) {
    if (!data || data.length === 0) {
        return Buffer.alloc(0);
    }

    const compressed = [];
    let currentByte = data[0];
    let count = 1;

    for (let i = 1; i < data.length; i++) {
        if (data[i] === currentByte && count < 255) {
            count++;
        } else {
            compressed.push(currentByte);
            compressed.push(count);
            currentByte = data[i];
            count = 1;
        }
    }

    // Push the last run
    compressed.push(currentByte);
    compressed.push(count);

    return Buffer.from(compressed);
}

// RLE decompression
function decompress(data) {
    if (!data || data.length === 0) {
        return Buffer.alloc(0);
    }

    const decompressed = [];
    let i = 0;

    while (i < data.length) {
        const byte = data[i];
        const count = data[i + 1];
        for (let j = 0; j < count; j++) {
            decompressed.push(byte);
        }
        i += 2;
    }

    return Buffer.from(decompressed);
}

module.exports = {
    compress,
    decompress
};
