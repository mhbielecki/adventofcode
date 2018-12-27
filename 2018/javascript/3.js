const fs = require('fs')

class Claim {
    constructor(id, leftOffset, topOffset, width, height) {
        this.id = id;
        this.leftOffset = parseInt(leftOffset);
        this.topOffset = parseInt(topOffset);
        this.width = parseInt(width);
        this.height = parseInt(height);
    }
}

fs.readFile('../input/3.txt', 'utf8', (err, contents) => {
    const claimLine = /#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)/; // #1 @ 1,3: 4x4
    const claims =
        contents
            .split("\r\n")
            .map(claim => {
                let m = claimLine.exec(claim);
                return new Claim(m[1], m[2], m[3], m[4], m[5])
            });

    const squareInches = {}
    const claimIds = new Set(claims.map(claim => claim.id));

    claims.forEach(claim => {
        for (let i = claim.leftOffset; i < claim.leftOffset + claim.width; i++) {
            for (let j = claim.topOffset; j < claim.topOffset + claim.height; j++) {
                let pair = `${i},${j}`;
                if (!squareInches[pair]) {
                    squareInches[pair] = { overlaps: 1, claim: claim.id };
                }
                else {
                    squareInches[pair].overlaps += 1
                    claimIds.delete(claim.id);
                    claimIds.delete(squareInches[pair].claim);
                }
            }
        }
    });

    let overLapping = 0;
    for (const [_, value] of Object.entries(squareInches)) {
        if (value.overlaps > 1) overLapping++;
    }

    console.log(overLapping);
    console.log(claimIds); // The one claim not having any overlaps
});
