"use strict";
import {setTerm, setDebug, appendTerm, appendDebug} from './bkit.js';
import * as bkbd from './bkbd.js';

const h1 = document.querySelector('h1');
const screen = document.querySelector('#screen');
const left = document.querySelector('#left');
const right = document.querySelector('#right');
const connectBtn = document.querySelector('#connectBtn');
const scancodeURL = 'http://localhost:8000/io/kbd/scancode?';
var serverEvents = null;

// Set initial header and button state for disconnected
stop();

// Attach DOM elements to be used for terminal and debug text logs
setTerm(screen);
setDebug(right);

// Configure the connect/hangup buton
connectBtn.addEventListener('click', e => {
    if (connectBtn.classList.contains('stopped')) {
        start();
    } else {
        stop();
    }
});

function stop() {
    // Stop POSTing keyboard events
    bkbd.removeKeyboardListener(document);
    // Close the Server-Sent Events connection
    if (serverEvents) {
        serverEvents.close();
    }
    // Update header and connection button
    h1.textContent = "No connection";
    connectBtn.textContent = "Connect to ui-sim (localhost:8000)";
    connectBtn.classList.add("stopped");
}

function start() {
    // Make sure any previous Server-Sent Events connection is closed
    if (serverEvents) {
        serverEvents.close();
    }
    // Start a new Server-Sent Events connection (events from ui-sim)
    serverEvents = new EventSource('/io/screen');
    serverEvents.addEventListener('open', e => {
        appendDebug('SSE /io/screen connected');
        console.log('SSE /io/screen connected');
    });
    serverEvents.addEventListener('message', e => {
        // This is for generic 'data: ...' messages
        appendTerm(e.data);
        console.log('SSE data', e.data);
    });
    serverEvents.addEventListener('term', e => {
        // This is for 'event: term\ndata: ...' messages
        appendTerm(e.data);
    });
    serverEvents.addEventListener('trace', e => {
        // This is for 'event: trace\ndata: ...' messages
        appendDebug(e.data);
        console.log('SSE trace', e.data);
    });
    serverEvents.addEventListener('error', e => {
        appendDebug('SSE /io/screen network error');
        console.log('SSE /io/screen network error');
        stop();
    });
    // Start POSTing keyboard up and down events to ui-sim at scancode URL
    bkbd.addKeyboardListener(document, scancodeURL);
    // Update header and connection button
    h1.textContent = "Connected to ui-sim (localhost:8000)";
    connectBtn.textContent = "Hang Up";
    connectBtn.classList.remove("stopped");
}
