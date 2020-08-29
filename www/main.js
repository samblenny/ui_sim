"use strict";
import * as bKit from './bkit.js';
import * as bKbd from './bkbd.js';

const h1 = document.querySelector('h1');
const screen = document.querySelector('#screen');
const left = document.querySelector('#left');
const right = document.querySelector('#right');
const connectBtn = document.querySelector('#connectBtn');
const scancodeURL = 'http://localhost:8000/io/kbd/scancode?';
var serverEvents = null;

stop();

connectBtn.addEventListener('click', e => {
    if (connectBtn.classList.contains('stopped')) {
        start();
    } else {
        stop();
    }
});

function stop() {
    // Stop POSTing keyboard events
    bKbd.removeKeyboardListener(document);
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
    serverEvents.addEventListener('open', e => console.log('SSE connection is open'));
    serverEvents.addEventListener('message', e => console.log('SSE message', e.data));
    serverEvents.addEventListener('error', e => {
        console.log('SSE error', e);
        stop();
    });
    // Start POSTing keyboard up and down events to ui-sim at scancode URL
    bKbd.addKeyboardListener(document, scancodeURL);
    // Update header and connection button
    h1.textContent = "Connected to ui-sim (localhost:8000)";
    connectBtn.textContent = "Hang Up";
    connectBtn.classList.remove("stopped");
}
