 const fs = require('fs');
const path = require('path');
const rle = require('./rle');
const lz = require('./lz');

function main() {
    const args = process.argv.slice(2);
    
    if (args.length !== 4) {
        console.error('Usage: node index.js compress|decompress <input_file> <output_file> --rle|--lz');
        process.exit(1);
    }

    const [operation, inputFile, outputFile, algorithm] = args;

    try {
        // Read input file
        const inputData = new Uint8Array(fs.readFileSync(inputFile));

        // Process data
        let result;
        if (operation === 'compress') {
            result = algorithm === '--rle' ? rle.compress(inputData) : lz.compress(inputData);
        } else if (operation === 'decompress') {
            result = algorithm === '--rle' ? rle.decompress(inputData) : lz.decompress(inputData);
        } else {
            throw new Error('Invalid operation. Use compress or decompress');
        }

        // Write output file
        fs.writeFileSync(outputFile, Buffer.from(result));
        console.log('Operation completed successfully!');
    } catch (error) {
        console.error('Error:', error.message);
        process.exit(1);
    }
}

// Run if this file is executed directly
if (require.main === module) {
    main();
}

module.exports = {
    rle,
    lz
};
