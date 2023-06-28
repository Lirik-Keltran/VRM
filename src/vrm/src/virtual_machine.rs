use crate::{comands::{Command}, vrm_values::Value};

#[derive(Debug)]
pub enum VMError {
    UsingNotInitMemory,
    NotCorrectType
}

pub struct VM {
    program: Vec<Command>,
    memory: Vec<Value>,
    cursor: usize,
}

impl std::fmt::Display for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, " Cursor - {} \n Memory - {:?} \n Program - {:?} \n", self.cursor, self.memory, self.program)
    }
}

pub type ExecuteResult = Result<(), VMError>;

impl VM {

    pub fn new(program: Vec<Command>) -> VM {
        VM{
            program,
            memory: vec![],
            cursor: 0,
        }
    }

    pub fn eat(&mut self) -> Option<ExecuteResult>{
        if self.program.len() > self.cursor {

            let command = &self.program[self.cursor];

            self.cursor += 1;

            let result = match command {
                Command::Add => self.add(),
                Command::Sub => self.sub(),
                Command::JMP(adress) => self.jump(*adress),
                Command::Push(value) => self.push(*value),
                Command::Stop => return None,
            };

            Some(result)
        } else {
            None
        }
    }

    fn add(&mut self) -> ExecuteResult {
        let left = self.memory.pop().ok_or(VMError::UsingNotInitMemory)?;
        let right = self.memory.pop().ok_or(VMError::UsingNotInitMemory)?;

        let value = (left + right).unwrap();
        let _ = self.push(value);

        Ok(())
    }

    fn sub(&mut self) -> ExecuteResult {
        let left = self.memory.pop().ok_or(VMError::UsingNotInitMemory)?;
        let right = self.memory.pop().ok_or(VMError::UsingNotInitMemory)?;

        let value = (left - right).unwrap();
        let _ = self.push(value);

        Ok(())
    }

    fn jump(&mut self, adress: usize) -> ExecuteResult {
        self.cursor = adress;
        Ok(())
    }

    fn push(&mut self, value: Value) -> ExecuteResult {
        self.memory.push(value);
        Ok(())
    }
}
