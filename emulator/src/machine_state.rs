use std::convert::TryInto;

use crate::instruction::InstructionParam;
use crate::instruction_set::{InstructionSize, Register};
use crate::{cpu, decoder};

pub struct MachineState {
    pub i0: i64,
    pub i1: i64,
    pub i2: i64,
    pub i3: i64,
    pub i4: i64,
    pub i5: i64,
    pub i6: i64,
    pub i7: i64,
    pub i8: i64,
    pub i9: i64,
    pub i10: i64,
    pub i11: i64,
    pub i12: i64,
    pub i13: i64,
    pub i14: i64,
    pub i15: i64,
    pub i16: i64,
    pub i17: i64,
    pub i18: i64,
    pub i19: i64,
    pub i20: i64,
    pub i21: i64,
    pub i22: i64,
    pub i23: i64,

    pub pc: i64,
    pub sp: i64,
    pub fp: i64,

    pub memory: Vec<u8>,

    pub halt: bool,
}

impl MachineState {
    pub fn new() -> MachineState {
        MachineState {
            i0: 0,
            i1: 0,
            i2: 0,
            i3: 0,
            i4: 0,
            i5: 0,
            i6: 0,
            i7: 0,
            i8: 0,
            i9: 0,
            i10: 0,
            i11: 0,
            i12: 0,
            i13: 0,
            i14: 0,
            i15: 0,
            i16: 0,
            i17: 0,
            i18: 0,
            i19: 0,
            i20: 0,
            i21: 0,
            i22: 0,
            i23: 0,

            pc: 0x10000,
            sp: 1048576, // stack is at the end of the memory, growing down
            fp: 0x0,
            memory: vec![0; 1048576],

            halt: false,
        }
    }

    pub fn run(&mut self, print_instr: bool) {
        loop {
            let (instr, offset) = decoder::decode(&self, print_instr);

            self.pc += offset;
            let result = cpu::execute(self, &instr);
            match result {
                Ok(()) => (),
                Err(err) => {
                    println!("Execution error: {:?}, {:?}", err, instr.instruction);
                    self.halt = true;
                }
            }
            if self.halt {
                break;
            }
        }
    }

    pub fn get_value(&self, instruction_param: &InstructionParam, size: &InstructionSize) -> i64 {
        let value = match instruction_param {
            InstructionParam::Immediate(value) => *value,
            InstructionParam::Register(register) => match register {
                Register::I0 => self.i0,
                Register::I1 => self.i1,
                Register::I2 => self.i2,
                Register::I3 => self.i3,
                Register::I4 => self.i4,
                Register::I5 => self.i5,
                Register::I6 => self.i6,
                Register::I7 => self.i7,
                Register::I8 => self.i8,
                Register::I9 => self.i9,
                Register::I10 => self.i10,
                Register::I11 => self.i11,
                Register::I12 => self.i12,
                Register::I13 => self.i13,
                Register::I14 => self.i14,
                Register::I15 => self.i15,
                Register::I16 => self.i16,
                Register::I17 => self.i17,
                Register::I18 => self.i18,
                Register::I19 => self.i19,
                Register::I20 => self.i20,
                Register::I21 => self.i21,
                Register::I22 => self.i22,
                Register::I23 => self.i23,
                Register::PC => self.pc,
                Register::SP => self.sp,
                Register::FP => self.fp,
            },
        };
        match size {
            InstructionSize::Zero => unreachable!("Can't use size zero to get value"),
            InstructionSize::OneByte => value as i8 as i64,
            InstructionSize::TwoByte => value as i16 as i64,
            InstructionSize::FourByte => value as i32 as i64,
            InstructionSize::EightByte => value,
        }
    }

    pub fn set_value(
        &mut self,
        value: i64,
        instruction_param: &InstructionParam,
        size: &InstructionSize,
    ) {
        let value = match size {
            InstructionSize::Zero => unreachable!("Can't use size zero to set value"),
            InstructionSize::OneByte => value as i8 as i64,
            InstructionSize::TwoByte => value as i16 as i64,
            InstructionSize::FourByte => value as i32 as i64,
            InstructionSize::EightByte => value,
        };
        match instruction_param {
            InstructionParam::Immediate(_value) => panic!("Cannot set value on immediate argument"),
            InstructionParam::Register(register) => match register {
                Register::I0 => self.i0 = value,
                Register::I1 => self.i1 = value,
                Register::I2 => self.i2 = value,
                Register::I3 => self.i3 = value,
                Register::I4 => self.i4 = value,
                Register::I5 => self.i5 = value,
                Register::I6 => self.i6 = value,
                Register::I7 => self.i7 = value,
                Register::I8 => self.i8 = value,
                Register::I9 => self.i9 = value,
                Register::I10 => self.i10 = value,
                Register::I11 => self.i11 = value,
                Register::I12 => self.i12 = value,
                Register::I13 => self.i13 = value,
                Register::I14 => self.i14 = value,
                Register::I15 => self.i15 = value,
                Register::I16 => self.i16 = value,
                Register::I17 => self.i17 = value,
                Register::I18 => self.i18 = value,
                Register::I19 => self.i19 = value,
                Register::I20 => self.i20 = value,
                Register::I21 => self.i21 = value,
                Register::I22 => self.i22 = value,
                Register::I23 => self.i23 = value,
                Register::PC => self.pc = value,
                Register::SP => self.sp = value,
                Register::FP => self.fp = value,
            },
        };
    }

    pub fn read_memory1(&self, address: i64) -> i8 {
        self.memory[address as usize] as i8
    }

    pub fn read_memory2(&self, address: i64) -> i16 {
        let value = &self.memory[address as usize..address as usize + 2];
        i16::from_le_bytes(value.try_into().unwrap())
    }

    pub fn read_memory4(&self, address: i64) -> i32 {
        let value = &self.memory[address as usize..address as usize + 4];
        i32::from_le_bytes(value.try_into().unwrap())
    }

    pub fn read_memory8(&self, address: i64) -> i64 {
        let value = &self.memory[address as usize..address as usize + 8];
        i64::from_le_bytes(value.try_into().unwrap())
    }

    pub fn write_memory1(&mut self, address: i64, value: i8) {
        self.memory[address as usize] = value as u8;
    }
    pub fn write_memory2(&mut self, address: i64, value: i16) {
        self.memory[address as usize..address as usize + 2].copy_from_slice(&value.to_le_bytes());
    }
    pub fn write_memory4(&mut self, address: i64, value: i32) {
        self.memory[address as usize..address as usize + 4].copy_from_slice(&value.to_le_bytes());
    }
    pub fn write_memory8(&mut self, address: i64, value: i64) {
        self.memory[address as usize..address as usize + 8].copy_from_slice(&value.to_le_bytes());
    }
    pub fn write_memory(&mut self, address: usize, data: &[u8]) {
        self.memory[address..address + data.len()].copy_from_slice(data);
    }
}
