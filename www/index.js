import { memory } from "chip8/chip8_bg";
import {Cpu} from "chip8";

const cpu = Cpu.new();
const displayPtr = cpu.read_display();
const display = new Uint8Array(memory.buffer, displayPtr, 32 * 64);

console.log(cpu, display);