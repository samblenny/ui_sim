"use strict";

var terminalElement = null;
var debugElement = null;

// Set the text terminal element
export function setTerm(el) {
    terminalElement = el;
}

// Set the debug log element
export function setDebug(el) {
    debugElement = el;
}

// Append plain text to terminal log and scroll so it is visible.
// This is uses `Node.textContent = ...` to escape user input (anti-XSS precaution).
export function appendTerm(unsafeMessage) {
    if (terminalElement) {
        let p = document.createElement('p');
        p.textContent = unsafeMessage;
        terminalElement.insertAdjacentElement('beforeend', p);
        terminalElement.scrollTop = terminalElement.scrollHeight;
    }
}

// Same as appendTerm, but for the debug log
export function appendDebug(unsafeMessage) {
    if (debugElement) {
        let p = document.createElement('p');
        p.textContent = unsafeMessage;
        debugElement.insertAdjacentElement('beforeend', p);
        debugElement.scrollTop = debugElement.scrollHeight;
    }
}
