const fs = require('fs');
var Stack = require('./Utils/Stack.js');

fs.readFile('../input/5.txt', 'utf8', (err, contents) => {
    const polymer = contents.trim().split("");
    const stack = new Stack();

    for (let i = 0, j = polymer.length; i < j; i++) {
        // Always 32 in diff between upper case and lower case of the same char (ascii)
        if (stack.peek() && (Math.abs(stack.peek().charCodeAt() - polymer[i].charCodeAt()) === 32)) { 
            stack.pop();
        }
        else {
            stack.push(polymer[i]);
        }
    }

    console.log(`Part 1: ${stack.length()}`);
});