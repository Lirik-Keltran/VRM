use comands::Command;
use virtual_machine::{VM, ExecuteResult};

pub mod comands;
pub mod vrm_values;
pub mod virtual_machine;

pub fn execute_program(program: Vec<Command>) -> impl Iterator<Item = ExecuteResult> + 'static {
    let mut vm = VM::new(program);
    std::iter::from_fn(move|| {
        println!("{}", vm);
        vm.eat()
    })
}