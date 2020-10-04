"use strict";

const canvas = document.querySelector('#canvas');
const preOut = document.querySelector('#preOut');
const bitmap = document.querySelector('#bitmap');
const imgSelect = document.querySelector('#imgSelect');
const ctx = canvas.getContext('2d');

window.addEventListener('load', init);

function init(e) {
    paintPngToCanvas();
    // Configure bitmap selector
    imgSelect.addEventListener('change', e => {
        bitmap.onload = paintPngToCanvas;
        bitmap.src = e.target.value;
    });
    // Configure canvas click detector
    canvas.addEventListener('click', doCanvasClick);
}

function paintPngToCanvas() {
    let h = bitmap.height;
    let w = bitmap.width;
    canvas.height = h;
    canvas.width = w;
    ctx.drawImage(bitmap, 0, 0, w, h);
    preOut.textContent = "";
}

function doCanvasClick(e) {
    let bbox = e.target.getBoundingClientRect();
    let x = e.clientX - Math.floor(bbox.left);
    let y = e.clientY - Math.floor(bbox.top);
    const border = 2;
    const columns = 16;
    // Grid is 16x16 boxes of 24x24px or 30x30px with 2px gutters and borders
    let gridSize = Math.floor((canvas.width - border) / columns);
    // Determine which grid box contains the click coordinate
    let row = Math.floor(y/gridSize);
    let col = Math.floor(x/gridSize);
    convertGlyphBoxToText(row, col);
}

function convertGlyphBoxToText(row, col) {
    const border = 2;
    const columns = 16;
    const rows = columns;
    if (row < 0 || row >= rows || col < 0 || col >= columns) {
        // Ignore clicks on grid border, etc.
        return;
    }
    let w = canvas.width;
    let h = canvas.height;
    var idat = ctx.getImageData(0, 0, w, h);
    // Get pixels for grid cell, converting from RGBA to 1-bit
    let grid = Math.floor((w - border) / columns);
    let pxMtrx = [];
    for (let y=(row*grid)+border; y<(row+1)*grid; y++) {
        let row = [];
        for (let x=(col*grid)+border; x<(col+1)*grid; x++) {
            let offset = ((w * y) + x) * 4;
            let r = idat.data[offset];
            row.push(r>0 ? 0 : 1);
        }
        pxMtrx.push(row);
    }
    // Trim left whitespace
    pxMtrx = matrixTranspose(pxMtrx);
    while (pxMtrx.length>0 && pxMtrx[0].reduce((a, b) => a+b) == 0) {
        pxMtrx.shift();
    }
    // Trim right whitespace
    pxMtrx.reverse();
    while (pxMtrx.length>0 && pxMtrx[0].reduce((a, b) => a+b) == 0) {
        pxMtrx.shift();
    }
    pxMtrx.reverse();
    pxMtrx = matrixTranspose(pxMtrx);
    // Show trimmed result as one ASCII char per pixel
    preOut.textContent = pxMtrx.map(
        row => row.map(col => col==0 ? "." : "#").join("")
    ).join("\n");
}

function matrixTranspose(matrix) {
    if (matrix.length < 1) {
        return matrix;
    }
    let w = matrix[0].length;
    let h = matrix.length;
    let transposed = [];
    for (let col=0; col<w; col++) {
        let trRow = [];
        for (let row=0; row<h; row++) {
            trRow.push(matrix[row][col]);
        }
        transposed.push(trRow);
    }
    return transposed;
}
