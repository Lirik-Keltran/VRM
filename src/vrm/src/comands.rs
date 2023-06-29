#[derive(Debug)]
pub struct Command {
    op_code : OpCode,
    argument: [u8;4],
    op_info : u8,
}

impl Command {
    pub fn pushi(value: i32) -> Command 
    {
        let argument: [u8;4] = unsafe { std::mem::transmute(value)};
        Command { 
            op_code: OpCode::Push, 
            argument,
            op_info: 0,
        }
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum OpCode {
    Push = 0,
    JMP,
    Add,
    Sub,
    Stop,
}