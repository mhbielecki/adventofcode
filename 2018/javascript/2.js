const fs = require('fs')
const _ = require('lodash');

fs.readFile('../input/2.txt', 'utf8', (err, contents) => {
    const codes = contents.split("\n").map(x => x.trim());

    // Part 1
    const charMaps = codes.map(code => {
        const charMap = {}
        code.split("").forEach(c => {
            charMap[c] ? charMap[c]++ : charMap[c] = 1;
        });
        return charMap;
    });

    const twos = charMaps.reduce(letter_count(2), 0);
    const threes = charMaps.reduce(letter_count(3), 0);
    console.log(`Result part 1: ${twos * threes}`);

    // Part 2
    codes.forEach((code, idx) => {
        let subSet = codes.slice(idx+1);
        subSet.forEach(sub => {
            let charPairs = _.zip(code.split(""), sub.split(""));
            let boxIdPairSummary = charPairs.reduce((acc, charPair) => {
                if (charPair[0] === charPair[1]) {
                    return { commonLetters: acc.commonLetters+=charPair[0], ...acc };
                }
                return { diff: acc.diff+=1, ...acc };

            }, { diff: 0, commonLetters: "" });
            if (boxIdPairSummary.diff === 1) {
                console.log(`Result part 2: ${boxIdPairSummary.commonLetters}`);
                return;
            }
        });
    });
});

const letter_count = (n) => {
    return (acc, curr) => {
        for (const [_, value] of Object.entries(curr)) {
            if (value === n) return acc + 1;
        }
        return acc;
    };
};