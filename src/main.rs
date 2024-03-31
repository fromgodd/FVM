use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

#[derive(Debug)]
enum Instruction {
    PUSH(i32),
    LOAD(i32),
    STRING(String),
    ADD,
    ADDSTR,
    PRINT,
    PRINTSTR,
}

struct FVM {
    stack: Vec<i32>,
    str_stack: Vec<String>,
}

impl FVM {
    fn new() -> Self {
        FVM {
            stack: Vec::new(),
            str_stack: Vec::new(),
        }
    }

    fn execute(&mut self, program: Vec<Instruction>) {
        for instruction in program {
            match instruction {
                Instruction::PUSH(value) => self.stack.push(value),
                Instruction::LOAD(value) => self.stack.push(value),
                Instruction::STRING(s) => self.str_stack.push(s),
                Instruction::ADD => {
                    let b = self.stack.pop().unwrap_or(0);
                    let a = self.stack.pop().unwrap_or(0);
                    self.stack.push(a + b);
                }
                Instruction::ADDSTR => {
                    let b = self.str_stack.pop().unwrap_or_else(|| String::new());
                    let a = self.str_stack.pop().unwrap_or_else(|| String::new());
                    self.str_stack.push(a + &b);
                }
                Instruction::PRINT => {
                    if let Some(value) = self.stack.pop() {
                        println!("{}", value);
                    } else {
                        println!("\x1b[31mStack is empty\x1b[0m");
                    }
                }
                Instruction::PRINTSTR => {
                    if let Some(s) = self.str_stack.pop() {
                        print!("{}", s);
                        io::stdout().flush().unwrap();
                    } else {
                        println!("\x1b[31mString stack is empty\x1b[0m");
                    }
                }
            }
        }
    }

    fn load_program(&mut self, filename: &str) -> io::Result<Vec<Instruction>> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        let mut program = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            match parts[0] {
                "PUSH" => {
                    if let Some(value) = parts.get(1) {
                        if let Ok(value) = value.parse::<i32>() {
                            program.push(Instruction::PUSH(value));
                        }
                    }
                }
                "LOAD" => {
                    if let Some(value) = parts.get(1) {
                        if let Ok(value) = value.parse::<i32>() {
                            program.push(Instruction::LOAD(value));
                        }
                    }
                }
                "STRING" => {
                    if let Some(s) = parts.get(1) {
                        program.push(Instruction::STRING(s.to_string()));
                    }
                }
                "ADD" => program.push(Instruction::ADD),
                "ADDSTR" => program.push(Instruction::ADDSTR),
                "PRINT" => program.push(Instruction::PRINT),
                "PRINTSTR" => program.push(Instruction::PRINTSTR),
                _ => {
                    println!("\x1b[31mUnknown instruction: {}\x1b[0m", parts[0]);
                }
            }
        }
        Ok(program)
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut vm = FVM::new();

    if args.len() != 2 {
        println!("\x1b[33mUsage: fvm <filename>\x1b[0m");
        return Ok(());
    }

    let filename = &args[1];
    let program = vm.load_program(filename)?;

    vm.execute(program);

    Ok(())
}
