const fs = require('fs')

fs.readFile('../input/1.txt', 'utf8', (err, contents) => {

    const frequencyChangeList = contents.split("\n").map(x => { return Number(x.trim())});
    const cycleFreqList = function* (idx) {
        while (true) {
            yield frequencyChangeList[idx];
            idx = ++idx % frequencyChangeList.length
        }
    }

    // Part 1
    const sum = frequencyChangeList.reduce((acc, curr) => acc + curr);
    console.log(`Result part 1: ${sum}`);

    // Part 2
    let currentFrequency = 0;
    const seenFrequencies = new Set()
    const frequencyGen = cycleFreqList(0);
    while (!seenFrequencies.has(currentFrequency)) {
        seenFrequencies.add(currentFrequency)
        currentFrequency += frequencyGen.next().value;
    }
    console.log(`Result part 2: ${currentFrequency}`);
});