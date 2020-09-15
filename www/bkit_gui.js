"use strict";
// bkit_gui provides a stack-oriented interpreted language for drawing, including:
// - VM opcodes to do integer math, draw shapes, and render text
// - Syntax for defining functions, utf8 strings, and bitmaps

// Token types (intended to work like Rust enum variants)
class TString    {constructor(v) {this.v=v;} toString() {return `TString(${this.v})`;}}
class TBitmap    {constructor(v) {this.v=v;} toString() {return `TBitmap(${this.v})`;}}
class TInteger   {constructor(v) {this.v=v;} toString() {return `TInteger(${this.v})`;}}
class TOpcode    {constructor(v) {this.v=v;} toString() {return `TOpcode(${this.v})`;}}
class TSymbol    {constructor(v) {this.v=v;} toString() {return `TSymbol(${this.v})`;}}
class TUnderflow {constructor(v) {this.v=null;} toString() {return "TUnderflow";}}

// Interpreter and VM state for parser, stack, and registers
class StackMachineContext {
    constructor(code, svg) {
        // Clean out previous contents of SVG target element (clear the screen)
        while(svg.firstChild) {
            svg.removeChild(svg.firstChild);
        }
        // Parser state
        this.code = code;                 // Input buffer
        this.i = 0;                       // Index to current position in input
        this.maxIndex = code.length - 1;  // Index to end of input
        this.error = null;
        // VM state
        this.svg = svg         // Screen
        this.stack = [];       // Stack
        this.x = 0;            // Current x coordinate
        this.y = 0;            // Current y coordinate
        this.markX = 0;        // Marked x coordinate
        this.markY = 0;        // Marked Y coordinate
        this.stroke = 1;       // 0:none, 1:blk, 2:wht
        this.fill = 0;         // 0:none, 1:blk, 2:wht
        this.traceOn = false;  // Debug tracing: true=enabled
        this.fnDefs = {};      // Function definitions
    }

    // Add a defined function
    defineFn(name, words) {this.fnDefs[name] = words;}

    // Stack push
    push(val) {if (val) {this.stack.push(val);}}

    // Stack pop
    pop() {
        let t = this.stack.pop();
        return (t !== undefined) ? t : new TUnderflow();
    }

    // Get character from code buffer
    charAt(index) {return this.code[index];}

    // Remember message for parsing error
    setError(errMsg) {if (!this.error) {this.error=errMsg;}}

    // Return string slice around where a parsing error occured
    codeAroundError(startOfDef, problemIndex) {
        const range = 20;
        let before = Math.max(0, startOfDef-range);
        let after = Math.min(this.maxIndex+1, problemIndex);
        return `"...${this.code.slice(before, after)}"`;
    }

    // Send info message to console log if trace level is high enough
    traceInfo(message, object) {
        if (this.traceLevel >= 1) {
            let stack = `  len(stack)=${this.stack.length}  `;
            if (this.traceLevel >= 3) {
                stack = this.stack.map(n => n.v);
            }
            let registers = `(x:${this.x},y:${this.y})`;
            console.log(registers, stack, message, object);
        }
    }

    // Send debug message to console log if trace level is high enough
    traceDebug(message, object) {
        if (this.traceLevel >= 2) {
            let stack = `  len(stack)=${this.stack.length}  `;
            if (this.traceLevel >= 3) {
                stack = this.stack.map(n => n.v);
            }
            let registers = `(x:${this.x},y:${this.y})`;
            console.log(registers, stack, message, object);
        }
    }
}

// Stack machine VM opcodes
const opcodes = {
    "+": (vm) => {
        let t=vm.pop(), s=vm.pop();
        if (!(t instanceof TInteger && s instanceof TInteger)) {
            vm.setError(`+: operand types: S=${s} T=${t}`);
            return;
        }
        vm.push(new TInteger(s.v + t.v));
    },
    "-": (vm) => {
        let t=vm.pop(), s=vm.pop();
        if (!(t instanceof TInteger && s instanceof TInteger)) {
            vm.setError(`-: operand types: S=${s} T=${t}`);
            return;
        }
        vm.push(new TInteger(s.v - t.v));
    },
    "*": (vm) => {
        let t=vm.pop(), s=vm.pop();
        if (!(t instanceof TInteger && s instanceof TInteger)) {
            vm.setError(`*: operand types: S=${s} T=${t}`);
            return;
        }
        vm.push(new TInteger(s.v * t.v));
    },
    "shr": (vm) => {
        let t=vm.pop();
        if (!(t instanceof TInteger)) {
            vm.setError(`shr: operand type: T=${t}`);
            return;
        }
        vm.push(new TInteger(t.v >> 1));
    },
    "dup": (vm) => {
        let t=vm.pop();
        if (t instanceof TUnderflow) {
            vm.setError("dup: underflow");
            return;
        }
        vm.push(t);
        vm.push(t);
    },
    "drop": (vm) => {
        let t=vm.pop();
        if (t instanceof TUnderflow) {
            vm.setError("drop: underflow");
        }
    },
    "swap": (vm) => {
        let t=vm.pop(), s=vm.pop();
        if (t instanceof TUnderflow || s instanceof TUnderflow) {
            vm.setError(`swap: underflow: S=${s} T=${t}`);
            return;
        }
        vm.push(t);
        vm.push(s);
    },
    "over": (vm) => {
        let t=vm.pop(), s=vm.pop();
        if (t instanceof TUnderflow || s instanceof TUnderflow) {
            vm.setError(`over: underflow: S=${s} T=${t}`);
            return;
        }
        vm.push(s);
        vm.push(t);
        vm.push(s);
    },
    "goxy": (vm) => {
        let t=vm.pop(), s=vm.pop();
        if (!(t instanceof TInteger && s instanceof TInteger)) {
            vm.setError(`goxy: operand types: S=${s} T=${t}`);
            return;
        }
        vm.x=s.v;
        vm.y=t.v;
    },
    "+xy": (vm) => {
        let t=vm.pop(), s=vm.pop();
        if (!(t instanceof TInteger && s instanceof TInteger)) {
            vm.setError(`+xy: operand types: S=${s} T=${t}`);
            return;
        }
        vm.x+=s.v;
        vm.y+=t.v;
    },
    "mark": (vm) => {
        if (!((typeof vm.x)==="number" && (typeof vm.y)==="number")) {
            vm.setError(`mark: xy type: x=${vm.x} y=${vm.y}`);
            return;
        }
        vm.markX=vm.x;
        vm.markY=vm.y;
    },
    "gomark": (vm) => {
        vm.x=vm.markX;
        vm.y=vm.markY;
    },
    "stroke": (vm) => {
        let t = vm.pop();
        if (!(t instanceof TInteger)) {
            vm.setError(`stroke: operand type: T=${t}`);
            return;
        }
        vm.stroke=t.v;
    },
    "fill": (vm) => {
        let t = vm.pop();
        if (!(t instanceof TInteger)) {
            vm.setError(`fill: operand type: T=${t}`);
            return;
        }
        vm.fill=t.v;
    },
    "txtC": (vm) => {
        let t=vm.pop();
        if (!(t instanceof TString)) {
            vm.setError(`txtC: operand type: T=${t}`);
            return;
        }
        textC(vm, t.v);
    },
    "txtL": (vm) => {
        let t=vm.pop();
        if (!(t instanceof TString)) {
            vm.setError(`txtL: operand type: T=${t}`);
            return;
        }
        textL(vm, t.v);
    },
    "rect": (vm) => {
        let dn=vm.pop(), rt=vm.pop();
        if (!(rt instanceof TInteger && dn instanceof TInteger)) {
            vm.setError(`rect: operand types: right=${rt} down=${dn}`);
            return;
        }
        rect(vm, rt.v, dn.v);
    },
    "image": (vm) => {
        let h=vm.pop(), w=vm.pop(), bits=vm.pop();
        if (!(h instanceof TInteger && w instanceof TInteger && bits instanceof TBitmap)) {
            vm.setError(`image: operand type: ${bits} wide=${w} high=${h}`);
            return;
        } else if (bits.v.length !== w.v * h.v) {
            vm.setError(`image: bitmap size: len(pixels)=${bits.v.length} w*h=${w.v*h.v}`);
            return;
        }
        image(vm, bits.v, w.v, h.v);
    },
    "trace": (vm) => {
        let t=vm.pop();
        if (!(t instanceof TInteger)) {
            vm.setError(`trace: operand type: T=${t}`);
            return;
        }
        vm.traceLevel=t.v;
    },
    "nop": (vm) => {}, // No effect, but useful for tracing
};

// SVG namespace is required to make document.createElementNS work for SVG
const SVG_NS = 'http://www.w3.org/2000/svg';

// Draw rectangle to VM screen
function rect(vm, rt, dn) {
    let r = document.createElementNS(SVG_NS, 'rect');
    if (vm.stroke > 0) {
        // compensate for SVG stroke going half outside of bounding box
        r.setAttribute('x', `${vm.x}.5`);
        r.setAttribute('y', `${vm.y}.5`);
        r.setAttribute('width', rt-1);
        r.setAttribute('height', dn-1);
    } else {
        r.setAttribute('x', `${vm.x}.5`);
        r.setAttribute('y', `${vm.y}.5`);
        r.setAttribute('width', rt-1);
        r.setAttribute('height', dn-1);
    }
    r.setAttribute('class', `s${vm.stroke} f${vm.fill}`);
    vm.svg.appendChild(r);
    vm.traceInfo('rect:', r);
}

// Draw centered text to VM screen
// Take text color (fill:#...) from stroke value, taking it as foreground color
function textC(vm, text) {
    let tc = document.createElementNS(SVG_NS, 'text');
    tc.setAttribute('x', vm.x);
    tc.setAttribute('y', vm.y);
    tc.setAttribute('class', `tc f${vm.stroke}`);
    tc.textContent = text;
    vm.svg.appendChild(tc);
    vm.traceInfo('textC:"', tc);
}

// Draw left aligned text to VM screen
function textL(vm, text) {
    let tl = document.createElementNS(SVG_NS, 'text');
    tl.setAttribute('x', vm.x);
    tl.setAttribute('y', vm.y);
    tl.setAttribute('class', `tl f${vm.stroke}`);
    tl.textContent = text;
    vm.svg.appendChild(tl);
    vm.traceInfo('textL:', tl);
}

// Draw bitmap image to VM screen
function image(vm, bits, w, h) {
    let pixelGroup = document.createElementNS(SVG_NS, 'g');
    pixelGroup.setAttribute('class', `s0 f1`);
    for (let y=0; y<h; y++) {
        for (let x=0; x<w; x++) {
            if (bits[y*w+x]==1) {
                let pixel = document.createElementNS(SVG_NS, 'rect');
                pixel.setAttribute('x', vm.x + x);
                pixel.setAttribute('y', vm.y + y);
                pixel.setAttribute('width', 1);
                pixel.setAttribute('height', 1);
                pixelGroup.appendChild(pixel);
            }
        }
    }
    vm.svg.appendChild(pixelGroup);
    vm.traceInfo('image:', pixelGroup);
}

// Interpret code string and render the results to an SVG element
export function run(code, svgElement) {
    // Interpret code
    let vm = new StackMachineContext(code, svgElement);
    for (vm.i=0; vm.i<=vm.maxIndex && !vm.error; vm.i++) {
        let c = vm.charAt(vm.i);
        if      (c === "#") {
            consumeComment(vm);                  // Comment
        } else if (" \t\r\n".includes(c)) {
            ;                                    // Whitespace (ignore it)
        } else if (c === "(") {
            vm.push(compileStringDef(vm));       // String
        } else if (c === "<") {
            vm.push(compileBitmapDef(vm));       // Bitmap
        } else if (c === ":") {
            compileFunctionDef(vm);              // Function
        } else {
            // Tokens reaching this branch should be number, symbol, or opcode
            let w = collectWord(vm);
            evalWord(vm, w, 0);
        }
    }
    if (vm.error) {
        console.warn("error:", vm.error);
    }
}

// Skip over Comment, consuming characters until end of line
function consumeComment(vm) {
    for (/* nop */; vm.i<=vm.maxIndex && !vm.error; vm.i++) {
        let c = vm.charAt(vm.i);
        if (c==="\n" || c==="\r" || vm.i===vm.maxIndex) {
            return;
        }
    }
}

// Compile String definition, consuming chars until first unescaped ')'
// Escape char is '\', so "\)" collects as ')'
function compileStringDef(vm) {
    let strBuf=[], iOrig=vm.i;
    for (vm.i++; vm.i<=vm.maxIndex && !vm.error; vm.i++) {
        let c = vm.charAt(vm.i);
        if (c==="\\" && vm.i<vm.maxIndex) {        // Collect escaped char
            vm.i++;
            let escapedC = vm.charAt(vm.i);
            strBuf.push(escapedC);
        } else if (c === ")") {                    // End of string
            return new TString(strBuf.join(""));
        } else {                                   // Collect regular char
            strBuf.push(c);
        }
    }
    // Bad syntax: Unclosed string, missing ')'
    let codeContext = vm.codeAroundError(iOrig, iOrig+40);
    vm.setError("string: missing ')': " + codeContext);
    return null;
}

// Compile Bitmap definition, consuming bit characters until '>'
function compileBitmapDef(vm) {
    let bitBuf=[], iOrig=vm.i;
    for (vm.i++; vm.i<=vm.maxIndex && !vm.error; vm.i++) {
        let c = vm.charAt(vm.i);
        if(c==="0" || c==="1") {
            bitBuf.push(c);                               // Collect bit
        } else if (c.match(/^[ \t\r\n]$/)) {
            continue;                                 // Skip whitespace
        } else if (c===">") {
            return new TBitmap(bitBuf);                 // End of bitmap
        } else {
            // Bad char in bitmap (not 0, 1, or whitespace)
            let codeContext = vm.codeAroundError(iOrig, vm.i+1);
            vm.setError("bitmap: syntax error: " + codeContext);
            return null;
        }
    }
    // Bad syntax: Unclosed bitmap, missing '>'
    let codeContext = vm.codeAroundError(iOrig, iOrig+40);
    vm.setError("bitmap: missing '>': " + codeContext);
    return null;
}

// Function definition: consume chars and collect words until ';'
function compileFunctionDef(vm) {
    let words=[], iOrig=vm.i;
    for (vm.i++; vm.i<=vm.maxIndex && !vm.error; vm.i++) {
        let c = vm.charAt(vm.i);
        if (c === "#") {                          // Comment
            consumeComment(vm);
        } else if (" \t\r\n".includes(c)) {       // Whitespace
            ;
        } else if (c === "(") {                   // String
            let str = compileStringDef(vm);
            words.push(str);
        } else if (c === "<") {                   // Bitmap
            let bits = compileBitmapDef(vm);
            words.push(bits);
        } else if (c === ";") {                   // End function def
            let name = words.shift();
            if (name === undefined ) {
                let codeContext = vm.codeAroundError(iOrig, vm.i+1);
                vm.setError("function: missing function name: " + codeContext);
            } else if (name instanceof TOpcode) {
                let codeContext = vm.codeAroundError(iOrig, vm.i+1);
                vm.setError(`function: cannot redefine keyword "${name.v}": ` + codeContext);
            } else if (!(name instanceof TSymbol)) {
                let codeContext = vm.codeAroundError(iOrig, vm.i+1);
                vm.setError(`function: name cannot be number, <bitmap>, or (string): ` + codeContext);
            } else {
                vm.traceDebug(`: ${name.v} ... ; = `, words);
                vm.defineFn(name.v, words);
            }
            return;
        } else {                                  // Int, name, or opcode
            let other = collectWord(vm);
            words.push(other);
        }
    }
    // Bad syntax: Unclosed function, missing ';'
    let codeContext = vm.codeAroundError(iOrig, iOrig+40);
    vm.setError("function: missing ';': " + codeContext);
    return;
}

// Collect integer, name, or opcode, consuming chars until first whitespace
function collectWord(vm) {
    let wordBuf=[], iOrig=vm.i;
    for (/* nop */; vm.i<=vm.maxIndex && !vm.error; vm.i++) {
        let c = vm.charAt(vm.i);
        if(vm.i===vm.maxIndex && !" \t\r\n".includes(c)) {
            // Make sure not to truncate the last character of the input buffer
            wordBuf.push(c);
        }
        if (" \t\r\n".includes(c) || vm.i===vm.maxIndex) {
            // Whitespace or end of buffer means end of word, but what kind?
            let w = wordBuf.join("");
            if (w.match(/^-?[0-9]+$/)) {                             // Integer
                return new TInteger(Number.parseInt(w));
            } else if (w.match(/^[0-9]+\./)) {                // Bad: no floats
                let codeContext = vm.codeAroundError(iOrig, vm.i+1);
                vm.setError("Bad syntax: no floats: " + codeContext);
                return null;
            } else if (opcodes[w]) {                                  // Opcode
                return new TOpcode(w);
            } else {
                return new TSymbol(w);          // Symbol (maybe function name)
            }
        } else {                    // Collect non-whitespace to form next word
            wordBuf.push(c);
        }
    }
    // Reaching here may mean there is no more input (an okay thing)
    let codeContext = vm.codeAroundError(iOrig, iOrig+40);
    vm.setError("collectWord: unexpected end of input: " + codeContext);
    return null;
}

function evalWord(vm, word, callDepth) {
    let maxDepth = 30;
    if (callDepth >= maxDepth) {
        console.warn("call stack too deep: "+word);
    } else if (word instanceof TString) {
        // Push strings
        vm.traceDebug('eval string:', word.v);
        vm.push(word);
    } else if (word instanceof TBitmap) {
        // Push bitmap
        vm.traceDebug('eval bitmap:', word.v);
        vm.push(word);
    } else if (word instanceof TInteger) {
        // Push integer
        vm.traceDebug('eval integer:', word.v);
        vm.push(word);
    } else if (word instanceof TOpcode) {
        // Opcodes get invoked through the table of opcode function pointers
        vm.traceDebug('eval opcode:', word.v);
        opcodes[word.v](vm);
    } else if (word instanceof TSymbol) {
        // Symbols get looked up... if they are functions, they get expanded and evaluated
        vm.traceDebug('eval symbol:', word.v);
        let fnWords = vm.fnDefs[word.v];
        if (fnWords) {
            for (let w of fnWords) {
                evalWord(vm, w, callDepth+1);
            }
        } else {
            console.warn('eval of undefined symbol:', word);
        }
    } else {
        console.warn('eval  ???:', word);
    }
}
