const crypto = require('crypto');

function generateHash(data) {
    return crypto.createHash('sha256').update(data).digest('hex');
}

for (let i = 0; i < 100; i++) {
    const hash = generateHash(`Tweet ${i}`);
    console.log(`Hash ${i}: ${hash}`);
}
