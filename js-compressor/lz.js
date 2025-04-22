// LZ77 compression
function compress(data) {
    const WINDOW_SIZE = 32768; // Increased window size for better compression
    const MIN_MATCH_LENGTH = 3;
    const compressed = [];
    let cursor = 0;

    while (cursor < data.length) {
        let maxMatchLength = 0;
        let matchOffset = 0;
        let matchFound = false;

        const start = Math.max(0, cursor - WINDOW_SIZE);
        const end = Math.min(cursor + WINDOW_SIZE, data.length);

        // Look for matches in the sliding window
        for (let lookBack = start; lookBack < cursor; lookBack++) {
            let length = 0;
            while (lookBack + length < cursor &&
                   cursor + length < end &&
                   data[lookBack + length] === data[cursor + length]) {
                length++;
            }

            if (length > maxMatchLength && length >= MIN_MATCH_LENGTH) {
                maxMatchLength = length;
                matchOffset = cursor - lookBack;
                matchFound = true;
            }
        }

        if (matchFound) {
            // Encode match: 0x01 + offset (2 bytes) + length (2 bytes)
            compressed.push(0x01);
            compressed.push((matchOffset >> 8) & 0xFF);
            compressed.push(matchOffset & 0xFF);
            compressed.push((maxMatchLength >> 8) & 0xFF);
            compressed.push(maxMatchLength & 0xFF);
            cursor += maxMatchLength;
        } else {
            // Encode literal: 0x00 + byte
            compressed.push(0x00);
            compressed.push(data[cursor]);
            cursor++;
        }
    }

    return Buffer.from(compressed);
}

// LZ77 decompression
function decompress(data) {
    const decompressed = [];
    let i = 0;

    while (i < data.length) {
        const marker = data[i];
        switch (marker) {
            case 0x00:
                // Literal: 0x00 + byte
                if (i + 1 < data.length) {
                    decompressed.push(data[i + 1]);
                    i += 2;
                } else {
                    i = data.length;
                }
                break;
            case 0x01:
                // Match: 0x01 + offset (2 bytes) + length (2 bytes)
                if (i + 4 < data.length) {
                    const offset = (data[i + 1] << 8) | data[i + 2];
                    const length = (data[i + 3] << 8) | data[i + 4];
                    
                    if (offset > 0 && length > 0 && offset <= decompressed.length) {
                        const start = decompressed.length - offset;
                        for (let k = 0; k < length; k++) {
                            if (start + k < decompressed.length) {
                                decompressed.push(decompressed[start + k]);
                            }
                        }
                    }
                    i += 5;
                } else {
                    i = data.length;
                }
                break;
            default:
                i = data.length;
        }
    }

    return Buffer.from(decompressed);
}

module.exports = {
    compress,
    decompress
};
