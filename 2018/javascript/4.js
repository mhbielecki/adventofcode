const fs = require('fs')
const _ = require('lodash');

class Record {
    constructor(timestamp, event) {
        this.timestamp = new Date(timestamp);
        this.event = event;
    }
}

class SleepyTime {
    constructor(from, to) {
        this.from = from;
        this.to = to;
    }
}

fs.readFile('../input/4.txt', 'utf8', (err, contents) => {

    const recordLine = /^\[(.*)\]\s(.*)/
    const records = 
        contents
            .split("\r\n")
            .map(r => {
                let m = recordLine.exec(r);
                return new Record(m[1], m[2]);
            })
            .sort((a, b) => {
                return a.timestamp - b.timestamp;
            });

    const guardRegex = /^Guard\s#(\d+).*/
    const guards = {}
    let currentGuardId;
    let prevFellAsleepTime;

    records.forEach(record => {
        let m = guardRegex.exec(record.event);
        if (m !== null) {
            currentGuardId = m[1]
        } else if (record.event === "falls asleep") {
            prevFellAsleepTime = record.timestamp;
        }
        else {
            if (!guards[currentGuardId]) {
                guards[currentGuardId] = {
                    id: currentGuardId,
                    totalMinsAsleep: getMinutesFromDateDiff(prevFellAsleepTime, record.timestamp),
                    sleepyTimes: [ new SleepyTime(prevFellAsleepTime, record.timestamp) ]
                };
            } else {
                guards[currentGuardId].totalMinsAsleep += getMinutesFromDateDiff(prevFellAsleepTime, record.timestamp);
                guards[currentGuardId].sleepyTimes.push(new SleepyTime(prevFellAsleepTime, record.timestamp));
            }
        }
    });

    const sleepiestGuard = _.maxBy(_.values(guards), "totalMinsAsleep");
    const minutesAsleep = sleepiestGuard.sleepyTimes.reduce(calcMinsAsleep, {});
    // Part 1-just console.log all the minutes asleep, pick the highest one (minute 48 with my input) and muliply it by the guard ID
    console.log("Part 1");
    console.log(`Guard Id: ${sleepiestGuard.id}`);
    console.log(minutesAsleep);

    // Part 2
    const part2Guards = _.values(guards).reduce((acc, g) => {
        let s = g.sleepyTimes.reduce(calcMinsAsleep, {});
        for (const [key, value] of Object.entries(s)) {
            if (!acc[key]) {
                acc[key] = { c: value, gid: g.id}
            }  else if (+acc[key].c < +value) {
                 acc[key] = { c: value, gid: g.id}
            }
        }
        return acc;
    }, {});

    console.log("Part 2");
    console.table(part2Guards); // Just check the table and find the minute with the highest sleep-count and guard
});

const getMinutesFromDateDiff = (dateA, dateB) => {
    return Math.round((((dateB - dateA) % 86400000) % 3600000) / 60000);
};

const calcMinsAsleep = (acc, s) => {
    let from = s.from.getMinutes();
    let to = s.to.getMinutes();
    for (let i = from; i < to; i++) {
        acc[i] = !acc[i] ? 1 : acc[i] + 1;
    }
    return acc;
};