use crate::{comands::{Command}, vrm_values::Value};

#[derive(Debug)]
pub enum VMError {
    UsingNotInitMemory,
    IncorrectAdress,
    NotCorrectType,
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
                Command::JMP => self.jump(),
                Command::Push => self.push(),
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
        self.memory.push(value);

        Ok(())
    }

    fn sub(&mut self) -> ExecuteResult {
        let left = self.memory.pop().ok_or(VMError::UsingNotInitMemory)?;
        let right = self.memory.pop().ok_or(VMError::UsingNotInitMemory)?;

        let value = (left - right).unwrap();
        self.memory.push(value);

        Ok(())
    }

    fn jump(&mut self) -> ExecuteResult {
        let value = self.memory.pop().ok_or(VMError::UsingNotInitMemory)?;

        match value {
            Value::Int(adress) => {
                if adress >= 0 {
                    self.cursor = adress as usize;
                    Ok(())
                } else {
                    Err(VMError::IncorrectAdress)
                }
            },
            _ => Err(VMError::NotCorrectType)
        }
    }

    fn push(&mut self) -> ExecuteResult {
        let value = self.memory.pop().ok_or(VMError::UsingNotInitMemory)?;
        self.memory.push(value);
        Ok(())
    }
}
