import { memory } from "chip8/chip8_bg";
import {Cpu, init_panic_hook} from "chip8";

init_panic_hook();
const cpu = Cpu.new();
const displayPtr = cpu.read_display();
const display = new Uint8Array(memory.buffer, displayPtr, 32 * 64);

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
      const disp2 = new Uint8Array(memory.buffer, displayPtr, 32 * 64);
      let text = '';
      for(let i = 0;i < 32 * 64;i++) {
        text += disp2[i];
          if (i > 0 && i % 64 === 0) {
              text += '\n';
          }
      }

      document.getElementById('display').innerHTML = text;
  }

  document.getElementById('rom').addEventListener('change', handleFileSelect, false);
  
  
  document.getElementById('tick').addEventListener('click', tick, false);