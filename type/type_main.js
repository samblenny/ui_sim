"use strict";

const canvas = document.querySelector('#canvas');
const preOut = document.querySelector('#preOut');
const bitmap = document.querySelector('#bitmap');
const imgSelect = document.querySelector('#imgSelect');
const ctx = canvas.getContext('2d');

window.addEventListener('load', init);

function init(e) {
    canvas.height = 257;
    canvas.width = 257;
    convertPngToText();
    // Configure bitmap selector
    imgSelect.addEventListener('change', e => {
        bitmap.onload = convertPngToText;
        bitmap.src = e.target.value;
        imgSelect.blur();
    });
}

function convertPngToText() {
    let h = bitmap.height;
    let w = bitmap.width;
    canvas.height = h;
    canvas.width = w;
    ctx.drawImage(bitmap, 0, 0);
    var idat = ctx.getImageData(0, 0, w, h);
    // Convert image data to text pixels
    let row_buf = [];
    for (let y=0; y<idat.height; y++) {
        let row = [];
        for (let x=0; x<idat.width; x++) {
            let offset = ((idat.width * y) + x) * 4;
            let r = idat.data[offset];
            row.push(r>0 ? "." : '#');
        }
        row_buf.push(row.join(""));
    }
    preOut.textContent = row_buf.join("\n");
}
