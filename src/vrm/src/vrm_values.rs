use pipeline::*;

use std::{ops};

use super::virtual_machine::VMError;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Int(i32),
    Float(f32),
}

fn to_float(num: f32) -> Value {
    Value::Float(num)
}

fn to_int(num: i32) -> Value {
    Value::Int(num)
}

impl ops::Sub<&Value> for &Value {
    type Output = Result<Value, VMError>;

    fn sub(self, right: &Value) -> Self::Output {
        match (self, right) {
            (Value::Float(left), Value::Float(right)) => {
                pipe!(
                    left-right
                    => to_float
                    => Ok
                )
            },
            (Value::Int(left), Value::Int(right)) => {
                pipe!(
                    left-right
                    => to_int
                    => Ok
                )
            },
            _ => Err(VMError::NotCorrectType)
        }
    }
}

impl ops::Add<&Value> for &Value {
    type Output = Result<Value, VMError>;

    fn add(self, right: &Value) -> Self::Output {
        match (self, right) {
            (Value::Float(left), Value::Float(right)) => {
                pipe!(
                    left+right
                    => to_float
                    => Ok
                )
            },
            (Value::Int(left), Value::Int(right)) => {
                pipe!(
                    left+right
                    => to_int
                    => Ok
                )
            },
            _ => Err(VMError::NotCorrectType)
        }
    }
}

impl ops::Sub<Value> for Value {
    type Output = Result<Value, VMError>;

    fn sub(self, right: Value) -> Self::Output {
        match (self, right) {
            (Value::Float(left), Value::Float(right)) => {
                pipe!(
                    left-right
                    => to_float
                    => Ok
                )
            },
            (Value::Int(left), Value::Int(right)) => {
                pipe!(
                    left-right
                    => to_int
                    => Ok
                )
            },
            _ => Err(VMError::NotCorrectType)
        }
    }
}

impl ops::Add<Value> for Value {
    type Output = Result<Value, VMError>;

    fn add(self, right: Value) -> Self::Output {
        match (self, right) {
            (Value::Float(left), Value::Float(right)) => {
                pipe!(
                    left+right
                    => to_float
                    => Ok
                )
            },
            (Value::Int(left), Value::Int(right)) => {
                pipe!(
                    left+right
                    => to_int
                    => Ok
                )
            },
            _ => Err(VMError::NotCorrectType)
        }
    }
}