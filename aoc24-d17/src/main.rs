use aoc24_tools::*;
use std::collections::HashSet;
use std::fs::read_to_string;

const DAY: u8 = 17;

fn main() {
    init_measurements!();
    print_header(DAY, "Chronospatial Computer");

    let data = read_to_string("input.txt").unwrap();
    let (part1, part2) = measure_total!({ solve(&data) });

    println!("Comma seperated output  (Part 1): {part1}");
    println!("Min. value for register A to output program (Part 2): {part2}");

    print_summary(DAY);
}

fn solve(input: &str) -> (String, u128) {
    let mut computer = Computer::parse(input);

    let part1 = computer.execute_program();
    let part2 = computer.find_reg_a_for_copy();

    (join(part1), part2.unwrap())
}

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
const ADV: char = '0';
const BXL: char = '1';
const BST: char = '2';
const JNZ: char = '3';
const BXC: char = '4';
const OUT: char = '5';
const BDV: char = '6';
const CDV: char = '7';

struct Computer {
    program: Vec<char>,
    registers: [u128; 3],
}

impl Computer {
    fn parse(input: &str) -> Computer {
        let lines = input.lines().collect::<Vec<&str>>();
        let reg_a = lines[0][12..].parse::<u128>().unwrap();
        let reg_b = lines[1][12..].parse::<u128>().unwrap();
        let reg_c = lines[2][12..].parse::<u128>().unwrap();
        let code = &lines[4][9..];
        Computer::new(reg_a, reg_b, reg_c, code)
    }

    fn new(register_a: u128, register_b: u128, register_c: u128, code: &str) -> Computer {
        let mut program = Vec::new();
        for (i, c) in code.chars().enumerate() {
            if i % 2 == 0 {
                program.push(c);
            }
        }
        Computer {
            program,
            registers: [register_a, register_b, register_c],
        }
    }

    fn execute_program(&mut self) -> Vec<char> {
        let mut output = Vec::new();
        let mut instruction_pointer = 0;
        while instruction_pointer < self.program.len() {
            let opcode = self.program[instruction_pointer];
            let operand = self.program[instruction_pointer + 1];

            self.do_instruction(opcode, operand, &mut output, &mut instruction_pointer);
        }
        output
    }

    fn find_reg_a_for_copy(&mut self) -> Option<u128> {
        // important to know: this is a 3 bit machine.
        // to get 1 output only 3 bits mater, the first 3.
        // to get the second last bits, the 3 bits after that mater, so on and so on.
        // but there are some shenanigans, so remember all possible options for each part.

        // bases contain values that have resulted in a valid output before, with a partial match at the end
        let mut bases = HashSet::new();
        bases.insert(0u128);
        // the idea is to try to find the last part of the program, if we find that,
        // we continue searching for the next instruction from the end,
        // so repeat for the total length of the program
        for i in 0..self.program.len() {
            // to find the lowest possible value for register A, sort the bases hashset and try to find new matches
            // from the lowest values
            let mut sorted_bases = bases.clone().into_iter().collect::<Vec<_>>();
            sorted_bases.sort();
            bases.clear(); //forget what we have found before, it's no longer relevant.

            for base in sorted_bases {
                let program = self.program.clone();
                // let's try to execute the program, by adding 3 more bits at the end of the register A
                for last_3_bits in 0..8 {
                    let reg_a = base * 8 + last_3_bits;
                    self.registers = [reg_a, 0, 0];

                    let output = self.execute_program(); //re-use the logic from part 1

                    // when the output is at least as long as the number of operations we are looking for
                    if output.len() >= (1 + i) && output.len() <= program.len() {
                        if program.ends_with(&output) {
                            // remember al valid values in this set:
                            bases.insert(reg_a);

                            if output.len() == program.len() {
                                return Some(reg_a);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn do_instruction(
        &mut self,
        opcode: char,
        operand: char,
        output: &mut Vec<char>,
        instruction_pointer: &mut usize,
    ) {
        let mut jumped = false;
        match opcode {
            ADV => self.division_on_a_with_pow2_on(operand), //0
            BXL => self.bitwise_xor_on_b_with(operand),      //1
            BST => self.set_b_to_modulo_8_of(operand),       //2
            JNZ => jumped = self.jump_when_a_non_zero_to(operand, instruction_pointer), //3
            BXC => self.set_b_to_bitwise_xor_on_b_with_c(operand), //4
            OUT => output.push(self.output_modulo_8_of(operand)), //5
            BDV => self.set_b_to_division_on_a_with_pow2_on(operand), //6, was unused in my case!
            CDV => self.set_c_to_division_on_a_with_pow2_on(operand), //7
            _ => panic!("This opcode should not exist in a 3-bit machine"),
        }
        if !jumped {
            *instruction_pointer += 2;
        }
    }

    fn read_operand(&self, operand: char) -> u128 {
        /*
            Combo operands 0 through 3 represent literal values 0 through 3.
            Combo operand 4 represents the value of register A.
            Combo operand 5 represents the value of register B.
            Combo operand 6 represents the value of register C.
            Combo operand 7 is reserved and will not appear in valid programs.
        */
        match operand {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => self.registers[A],
            '5' => self.registers[B],
            '6' => self.registers[C],
            '7' => panic!("Reserved for later use?"),
            _ => panic!("This should not exist in a 3-bit machine"),
        }
    }

    fn division_on_a_with_pow2_on(&mut self, operand: char) {
        // The adv instruction (opcode 0) performs division.
        // The numerator is the value in the A register.
        let numerator = self.registers[A];
        // The denominator is found by raising 2 to the power of the instruction's combo operand.
        let denominator = 2_u128.pow(self.read_operand(operand) as u32);
        // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
        // The result of the division operation is truncated to an integer and then written to the A register.
        self.registers[A] = numerator / denominator;
    }

    fn set_b_to_division_on_a_with_pow2_on(&mut self, operand: char) {
        /* The bdv instruction (opcode 6) works exactly like the adv instruction
        except that the result is stored in the B register.
        (The numerator is still read from the A register.) */
        let numerator = self.registers[A];
        let denominator = 2_u128.pow(self.read_operand(operand) as u32);
        self.registers[B] = numerator / denominator;
    }

    fn set_c_to_division_on_a_with_pow2_on(&mut self, operand: char) {
        /* The cdv instruction (opcode 7) works exactly like the adv instruction
        except that the result is stored in the C register.
        (The numerator is still read from the A register.) */
        let numerator = self.registers[A];
        let denominator = 2_u128.pow(self.read_operand(operand) as u32);
        self.registers[C] = numerator / denominator;
    }

    fn bitwise_xor_on_b_with(&mut self, operand: char) {
        // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand,
        // then stores the result in register B.
        self.registers[B] = self.registers[B] ^ operand.to_digit(10).unwrap() as u128;
    }

    fn set_b_to_modulo_8_of(&mut self, operand: char) {
        // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
        // (thereby keeping only its lowest 3 bits), then writes that value to the B register.
        self.registers[B] = self.read_operand(operand) % 8;
    }

    fn jump_when_a_non_zero_to(&mut self, operand: char, instruction_pointer: &mut usize) -> bool {
        // The jnz instruction (opcode 3) does nothing if the A register is 0.
        if self.registers[A] == 0 {
            false
        } else {
            // However, if the A register is not zero,
            // it jumps by setting the instruction pointer to the value of its literal operand;
            // if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
            *instruction_pointer = operand.to_digit(10).unwrap() as usize;
            true
        }
    }

    fn set_b_to_bitwise_xor_on_b_with_c(&mut self, _operand: char) {
        // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
        // then stores the result in register B.
        self.registers[B] = self.registers[B] ^ self.registers[C];
        // (For legacy reasons, this instruction reads an operand but ignores it.)
    }

    fn output_modulo_8_of(&self, operand: char) -> char {
        // The out instruction (opcode 5) calculates the value of its combo operand modulo 8,
        let mod8 = self.read_operand(operand) % 8u128;
        // then outputs that value.
        mod8.to_string().chars().nth(0).unwrap()
        // (If a program outputs multiple values, they are separated by commas.)
    }
}

fn join(chars: Vec<char>) -> String {
    let mut result = String::new();
    for (i, &c) in chars.iter().enumerate() {
        if i > 0 {
            result.push(',');
        }
        result.push(c);
    }
    result
}
