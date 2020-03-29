import { memory } from "chip8/chip8_bg";
import { Cpu, init_panic_hook } from "chip8";

const displayWidth = 64;
const displayHeight = 32;

let animationId = null;
const pixelSize = 10;

let displayElement = null;

const cpu = Cpu.new();
const displayPtr = cpu.display_reference();
const display = new Uint8Array(
  memory.buffer,
  displayPtr,
  displayHeight * displayWidth
);

export const pc = new Uint16Array(
  memory.buffer,
  cpu.pc_reference(),
  1
)

init_panic_hook();

export function initDisplay(displayId) {
    displayElement = document.getElementsByClassName(displayId)[0];
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

export function loadRomFromFile(romFile, readerResult) {

  const u8Buffer = romToU8Array(readerResult)

  console.log(u8Buffer)
  cpu.init(u8Buffer)
  //renderLoop()
}

export function romToU8Array(data) {
  const buffer = new ArrayBuffer(3584); //todo:; get length form chip8
  const u8Buffer = new Uint8Array(buffer);

  const romData = new Uint8Array(data);
  for (let i = 0; i < romData.length; i++) {
    u8Buffer[i] = romData[i]
  }

  return u8Buffer;
}

export function tick() {
  cpu.emulate_cycle();
  //console.log(cpu.read_pc());

  // for(let i = 0;i < 32 * 64;i++) {
  // text += disp2[i];
  //     if (i > 0 && i % 64 === 0) {
  //         text += '\n';
  //     }
  // }
  render(display);

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

// render loop

const renderLoop = () => {
  // console.log(cpu.read_pc(), 'pc')
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
export function keydown(keyCode) {
  if (16 > keyCode > 0) {
    cpu.key_down(keyCode);
  }
  console.log('down', keyCode);
}

export function keyup(keyCode) {
  if (16 > keyCode > 0) {
    cpu.key_up(keyCode);
  }
  console.log('up', keyCode);
}
