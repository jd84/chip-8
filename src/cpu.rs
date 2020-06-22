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
                    0x5 => self.sub_xy(x, y),
                    _ => unimplemented!("opcode {:04x}", opcode),
                },
                _ => unimplemented!("opcode {:04x}", opcode),
            }
        }
    }

    /// Resets the internal state and clears all memory
    pub fn reset(&mut self) {
        self.registers = [0; 16];
        self.memory = [0; 4096];
        self.stack = [0; 16];
        self.position_in_memory = 0;
        self.stack_pointer = 0;
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

    /// Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
    fn add_xy(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] > (0xFF - self.registers[y as usize]) {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
            self.registers[x as usize] += self.registers[y as usize];
        }
    }

    /// VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
    fn sub_xy(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] < (0x00 + self.registers[y as usize]) {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
            self.registers[x as usize] -= self.registers[y as usize];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_xy() {
        let mut cpu = Cpu::default();
        cpu.registers[0x0] = 5;
        cpu.registers[0x1] = 10;

        cpu.memory[0x000] = 0x80;
        cpu.memory[0x001] = 0x14;

        cpu.run();

        assert_eq!(15, cpu.registers[0x0]);
        assert_eq!(0, cpu.registers[0xF]);
        cpu.reset();

        cpu.registers[0x0] = 255;
        cpu.registers[0x1] = 1;

        cpu.memory[0x000] = 0x80;
        cpu.memory[0x001] = 0x14;
        cpu.run();

        assert_eq!(1, cpu.registers[0xF]);
    }

    #[test]
    fn test_sub_xy() {
        let mut cpu = Cpu::default();
        cpu.registers[0x0] = 10;
        cpu.registers[0x1] = 6;

        cpu.memory[0x000] = 0x80;
        cpu.memory[0x001] = 0x15;
        cpu.run();

        assert_eq!(4, cpu.registers[0x0]);
        assert_eq!(0, cpu.registers[0xF]);
        cpu.reset();

        cpu.registers[0x0] = 0;
        cpu.registers[0x1] = 1;

        cpu.memory[0x000] = 0x80;
        cpu.memory[0x001] = 0x15;
        cpu.run();

        assert_eq!(1, cpu.registers[0xF]);
    }

    #[test]
    fn test_assign() {
        let mut cpu = Cpu::default();
        cpu.registers[0x0] = 6;
        cpu.registers[0x1] = 10;

        cpu.memory[0x000] = 0x80;
        cpu.memory[0x001] = 0x10;

        cpu.run();

        assert_eq!(10, cpu.registers[0x0]);
    }

    #[test]
    fn test_call_and_ret() {
        let mut cpu = Cpu::default();
        cpu.registers[0x0] = 5;
        cpu.registers[0x1] = 10;

        cpu.memory[0x000] = 0x21;
        cpu.memory[0x001] = 0x00;
        cpu.memory[0x002] = 0x21;
        cpu.memory[0x003] = 0x00;

        cpu.memory[0x100] = 0x80;
        cpu.memory[0x101] = 0x14;
        cpu.memory[0x102] = 0x00;
        cpu.memory[0x103] = 0xEE;

        cpu.run();

        assert_eq!(25, cpu.registers[0x0]);
    }
}