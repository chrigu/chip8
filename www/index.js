import { memory } from "chip8/chip8_bg";
import {Cpu, init_panic_hook} from "chip8";

const displayWidth = 64;
const displayHeight = 32;
const pixelSize = 4;

init_panic_hook();
const cpu = Cpu.new();
const displayPtr = cpu.read_display();
const display = new Uint8Array(memory.buffer, displayPtr, displayHeight * displayWidth);

var displayElement = document.getElementsByClassName('display')[0];
console.log(displayElement);
const context = displayElement.getContext("2d");

function handleFileSelect(evt) {
    var files = evt.target.files; // FileList object

    // files is a FileList of File objects. List some properties.
    var output = [];
    for (var i = 0, f; f = files[i]; i++) {
      output.push('<li><strong>', escape(f.name), '</strong> (', f.type || 'n/a', ') - ',
                  f.size, ' bytes, last modified: ',
                  f.lastModifiedDate ? f.lastModifiedDate.toLocaleDateString() : 'n/a',
                  '</li>');
    }
    document.getElementById('list').innerHTML = '<ul>' + output.join('') + '</ul>';


    const buffer = new ArrayBuffer(3584);
    const u8Buffer = new Uint8Array(buffer);

    var reader = new FileReader();
    reader.onload = (function(theFile) {

        const romData = new Uint8Array(reader.result);
        for (let i = 0;i < romData.length;i++) {
            u8Buffer[i] = romData[i];
        }
        console.log(files, theFile, rom, u8Buffer);

        console.log(cpu, display)
        cpu.init(u8Buffer);
        console.log(cpu)

    })

    reader.readAsArrayBuffer(files[0])

}

function tick () {
    cpu.emulate_cycle();
    console.log(cpu.read_pc());
    const disp2 = new Uint8Array(memory.buffer, displayPtr, displayWidth * displayHeight);
    let text = '';
    for(let i = 0;i < 32 * 64;i++) {
    text += disp2[i];
        if (i > 0 && i % 64 === 0) {
            text += '\n';
        }
    }
    render(disp2);

    document.getElementById('display').innerHTML = text;
}

function render (pixels) {
    for(let i = 0;i < displayHeight * displayWidth;i++) {
        let x = pixelSize * (i % displayWidth);
        let y = pixelSize * Math.floor(i / displayWidth);
        drawPixel(x, y, pixels[i]);
    }
}

function drawPixel(x, y, set) {
    context.fillStyle = set ? 'rgb(76, 179, 91)' : 'rgb(0, 0, 0)';
    context.fillRect(x, y, pixelSize, pixelSize); 
}

document.getElementById('rom').addEventListener('change', handleFileSelect, false);
document.getElementById('tick').addEventListener('click', tick, false);