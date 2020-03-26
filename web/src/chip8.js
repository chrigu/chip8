import { memory } from "chip8/chip8_bg";
import { Cpu, init_panic_hook } from "chip8";

const displayWidth = 64;
const displayHeight = 32;

let animationId = null;
const pixelSize = 10;

const cpu = Cpu.new();
const displayPtr = cpu.read_display();
const display = new Uint8Array(
  memory.buffer,
  displayPtr,
  displayHeight * displayWidth
);

init_panic_hook();

export function initDisplay(displayId) {
    let displayElement = document.getElementsByClassName(displayId)[0];
    const dpr = window.devicePixelRatio || 1;
    // Get the size of the canvas in CSS pixels.
    const rect = displayElement.getBoundingClientRect();
    displayElement.width = rect.width * dpr;
    displayElement.height = rect.height * dpr;

    const ctx = displayElement.getContext("2d");
    // Scale all drawing operations by the dpr, so you
    // don't have to worry about the difference.
    ctx.scale(dpr, dpr);
}

export function loadRomFromFile(romFile, reader) {
  const romData = new Uint8Array(reader.result);
  for (let i = 0; i < romData.length; i++) {
    u8Buffer[i] = romData[i];
  }

  cpu.init(u8Buffer);
  renderLoop();

  //reader.readAsArrayBuffer(files[0]);
}




function tick() {
  cpu.emulate_cycle();
  //console.log(cpu.read_pc());
  const disp2 = new Uint8Array(
    memory.buffer,
    displayPtr,
    displayWidth * displayHeight
  );
  let text = "";
  // for(let i = 0;i < 32 * 64;i++) {
  // text += disp2[i];
  //     if (i > 0 && i % 64 === 0) {
  //         text += '\n';
  //     }
  // }
  render(disp2);

  //document.getElementById('display').innerHTML = text;
}

function render(pixels) {
  const ctx = displayElement.getContext("2d");
  for (let i = 0; i < displayHeight * displayWidth; i++) {
    let x = pixelSize * (i % displayWidth);
    let y = pixelSize * Math.floor(i / displayWidth);
    drawPixel(x, y, pixels[i], ctx);
  }
}

function drawPixel(x, y, set, ctx) {
  ctx.fillStyle = set ? "rgb(76, 179, 91)" : "rgb(0, 0, 0)";
  ctx.fillRect(x, y, pixelSize, pixelSize);
}

// document.getElementById("tick").addEventListener("click", tick, false);
// document.getElementById("pause").addEventListener("click", pause, false);

// render loop

const renderLoop = () => {
  tick();
  animationId = requestAnimationFrame(renderLoop);
};

export function pause () {
  cancelAnimationFrame(animationId);
  animationId = null;
};

function isPaused () {
  return animationId === null;
}

// keys
// document.addEventListener("keydown", keydown);
// document.addEventListener("keyup", keyup);

export function keydown(e) {
  let keyCode = mapKey(e.keyCode);
  if (keyCode > 0) {
    cpu.key_down(keyCode);
  }
  console.log('down', e.key, keyCode);
}

export function keyup(e) {
  let keyCode = mapKey(e.keyCode);
  if (keyCode > 0) {
    cpu.key_up(keyCode);
  }
  console.log('up', e.key, keyCode);
}

function mapKey(keyCode) {
  let chipKeyCode = -1;
  switch (keyCode) {
    case 49: // 1
      chipKeyCode = 1;
      break;
    case 50: // 2
      chipKeyCode = 2;
      break;
    case 51: // 3
      chipKeyCode = 3;
      break;
    case 81: // q
      chipKeyCode = 4;
      break;
    case 87: // w
      chipKeyCode = 5;
      break;
    case 69: // e
      chipKeyCode = 6;
      break;
    case 65: // a
      chipKeyCode = 7;
      break;
    case 83: // s
      chipKeyCode = 8;
      break;
    case 68: // d
      chipKeyCode = 9;
      break;
      case 89: // y
      chipKeyCode = 10;
      break;
    case 88: // x
      chipKeyCode = 11;
      break;
    case 67: // c
      chipKeyCode = 12;
      break;
    default:
      chipKeyCode = -1;
  }
  return chipKeyCode;
}



