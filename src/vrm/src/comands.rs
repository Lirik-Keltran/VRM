use crate::vrm_values::Value;

pub enum Commands {
    Push(Value),
    JMP(u32),
    Add,
    Sub,
}