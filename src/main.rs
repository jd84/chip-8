mod cpu;

use cpu::Cpu;

fn main() {
    let mut cpu = Cpu::default();

    cpu.registers[0] = 255;
    cpu.registers[1] = 1;

    cpu.memory[0x000] = 0x80;
    cpu.memory[0x001] = 0x14;
    cpu.run();

    println!("carry flag = {}", cpu.registers[0xF]);
}
