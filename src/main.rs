use vrm::{self, comands::Command, vrm_values::Value, virtual_machine::ExecuteResult};

fn main() {
    let program = vec![
        Command::JMP(2),
        Command::Push(Value::Int(2)),
        Command::Push(Value::Int(3)),
        Command::Push(Value::Int(4)),
        Command::Add,
    ];
    let _: ExecuteResult = vrm::execute_program(program).collect();
}