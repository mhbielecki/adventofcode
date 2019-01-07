const fs = require('fs');

class InstructionNode {

    constructor(id) {
        this.id = id;
        this.dependencies = [];
    }

    addDependency(dep) {
        this.dependencies.push(dep);
    }

    hasNoDependencies() {
        return this.dependencies.length === 0;
    }
}

fs.readFile('../input/7.txt', 'utf8', (err, contents) => {
    const instructionLines = contents.split("\n");
    const instructionLineRegex = /^Step ([A-Z]) must be finished before step ([A-Z]) can begin/
    const instructions = {};

    instructionLines.forEach(instruction => {
        let m = instructionLineRegex.exec(instruction);
        if (m === null) throw new Error(`Something is wrong with the input: ${instruction}`);
        let from = m[1], to = m[2];

        if (!instructions[from]) {
            instructions[from] = new InstructionNode(from);
        }
        if (!instructions[to]) {
            let n = new InstructionNode(to);
            n.addDependency(instructions[from]);
            instructions[to] = n;
        } else {
            instructions[to].addDependency(instructions[from]);
        }
    });

    console.log(instructions);
    
    // Part 1 - USe topological sorting of the graph to find the sorted order of the instructions
});
