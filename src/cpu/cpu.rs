/// The emulated Chip-8 cpu.
///
/// NNN: address
/// NN: 8-bit constant
/// N: 4-bit constant
/// X and Y: 4-bit register identifier
/// PC : Program Counter
/// I : 16bit register (For memory address) (Similar to void pointer)
/// VN: One of the 16 available variables. N may be 0 to F (hexadecimal)
pub struct Cpu {
    pub registers: [u8; 16],
    pub memory: [u8; 4096],
    position_in_memory: usize,
    stack: [u16; 16],
    stack_pointer: usize,
}

impl Default for Cpu {
    fn default() -> Cpu {
        Cpu {
            registers: [0; 16],
            memory: [0; 4096],
            position_in_memory: 0,
            stack: [0; 16],
            stack_pointer: 0,
        }
    }
}

impl Cpu {
    /// Process all opcodes until 0x0000 is reached.
    pub fn run(&mut self) {
        loop {
            let op_byte1 = self.memory[self.position_in_memory] as u16;
            let op_byte2 = self.memory[self.position_in_memory + 1] as u16;
            let opcode = op_byte1 << 8 | op_byte2;

            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let op_minor = (opcode & 0x000F) as u8;
            let addr = opcode & 0x0FFF;

            self.position_in_memory += 2;

            match opcode {
                0x0000 => return,
                0x00EE => self.ret(),
                0x2000..=0x2FFF => self.call(addr),
                0x8000..=0x8FFF => match op_minor {
                    0x0 => self.assign(x, y),
                    0x4 => self.add_xy(x, y),
                    0x7 => self.sub_xy(x, y),
                    _ => unimplemented!("opcode {:04x}", opcode),
                },
                _ => unimplemented!("opcode {:04x}", opcode),
            }
        }
    }

    /// Perform a jump and calls subroutine
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow");
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    /// Returns from subroutine
    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        self.position_in_memory = self.stack[self.stack_pointer] as usize;
    }

    /// Sets Vx to the value of Vy
    fn assign(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize];
    }

    /// Math add Vy to Vx
    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }

    /// Math subtract Vx from Vy. Vx=Vy-Vx
    fn sub_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize] - self.registers[x as usize];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_xy() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 5;
        cpu.registers[1] = 10;

        cpu.memory[0x000] = 0x80;
        cpu.memory[0x001] = 0x14;

        cpu.run();

        assert_eq!(15, cpu.registers[0]);
    }

    #[test]
    fn test_sub_xy() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 6;
        cpu.registers[1] = 10;

        cpu.memory[0x000] = 0x80;
        cpu.memory[0x001] = 0x17;

        cpu.run();

        assert_eq!(4, cpu.registers[0]);
    }

    #[test]
    fn test_assign() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 6;
        cpu.registers[1] = 10;

        cpu.memory[0x000] = 0x80;
        cpu.memory[0x001] = 0x10;

        cpu.run();

        assert_eq!(10, cpu.registers[0]);
    }

    #[test]
    fn test_call_and_ret() {
        let mut cpu = Cpu::default();
        cpu.registers[0] = 5;
        cpu.registers[1] = 10;

        cpu.memory[0x000] = 0x21;
        cpu.memory[0x001] = 0x00;
        cpu.memory[0x002] = 0x21;
        cpu.memory[0x003] = 0x00;

        cpu.memory[0x100] = 0x80;
        cpu.memory[0x101] = 0x14;
        cpu.memory[0x102] = 0x00;
        cpu.memory[0x103] = 0xEE;

        cpu.run();

        assert_eq!(25, cpu.registers[0]);
    }
}
