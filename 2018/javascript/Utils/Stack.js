class Stack {
    constructor() {
        this.s = [];
    }

    push(elem) {
        this.s.push(elem)
    }

    pop() {
        return this.s.pop();
    }

    peek() {
        return this.s[this.s.length-1];
    }

    length() {
        return this.s.length;
    }

    print() {
        console.log(this.s);
    }
}

module.exports = Stack;