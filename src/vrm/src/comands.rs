use crate::vrm_values::Value;

#[derive(Debug)]
pub enum Command {
    Push(Value),
    JMP(usize),
    Add,
    Sub,
    Stop,
}