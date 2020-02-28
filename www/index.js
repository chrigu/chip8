import { memory } from "chip8/chip8_bg";
import {Cpu} from "chip8";

const cpu = Cpu.new();
const displayPtr = cpu.read_display();
const display = new Uint8Array(memory.buffer, displayPtr, 32 * 64);

console.log(cpu, display);

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


    const buffer = new ArrayBuffer(3583);
    const u8Buffer = new Uint8Array(buffer);

    var reader = new FileReader();
    reader.onload = (function(theFile) {

        const romData = new Uint8Array(reader.result);
        for (let i = 0;i < romData.length;i++) {
            u8Buffer[i] = romData[i];
        }

        console.log(files, theFile, rom, u8Buffer);

    })

    reader.readAsArrayBuffer(files[0])

    
  }

  document.getElementById('rom').addEventListener('change', handleFileSelect, false);