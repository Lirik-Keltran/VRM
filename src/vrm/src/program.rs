use std::error::Error;

use crate::{comands::Commands, vrm_values::Value};

#[derive(Debug)]
pub enum VMError {
    NotInitMemory,
}

pub struct VM<'a> {
    program: Vec<Commands>,
    memory: Vec<&'a Value>,
    cursor: u32
}

impl<'a> VM<'a> {
    pub fn new(program: Vec<Commands>) -> VM<'a> {
        VM{
            program,
            memory: vec![],
            cursor: 0
        }
    }

    pub fn execute(&mut self) -> Result<Value, VMError>
    {
        for command in &self.program {
            match command {
                Commands::Add => self.add(),
                Commands::Sub => self.sub(),
                Commands::JMP(adress) => self.jump(adress),
                Commands::Push(value) => self.push(value),
            }
        }
        Err(VMError::NotInitMemory)
    }

    fn add(&self) {
        todo!()
    }

    fn sub(&self) {
        todo!()
    }

    fn jump(&self, adress: &u32) {
        todo!()
    }

    fn push(&self, value: &Value) {
        todo!()
    }
}
