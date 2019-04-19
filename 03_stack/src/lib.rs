use std::result;

pub enum Opcode {
    Push(i64),
    Add,
    Sub,
    Mul,
    Div,
    Done,
}

pub struct Program {
    program: Vec<Opcode>,
    stack: Vec<i64>,
}

#[derive(Debug)]
pub enum ProgramError {
    DivisionByZero,
    StackUnderflow,
}

type Result<T> = result::Result<T, ProgramError>;

macro_rules! make_op {
    ($code:expr, $op:tt) => {{
        if let Some(a) = $code.stack.pop() {
            if let Some(b) = $code.stack.pop() {
                $code.stack.push(b $op a);
                None
            } else { Some(ProgramError::StackUnderflow) }
        } else { Some(ProgramError::StackUnderflow) }
    }
}}

pub fn interpret(program: Vec<Opcode>) -> Result<i64> {
    let mut code = Program {
        program,
        stack: Vec::new(),
    };

    for op in code.program {
        if let Some(err) = match op {
            Opcode::Push(i) => {
                code.stack.push(i);
                None
            }
            Opcode::Mul => make_op!(code, *),
            Opcode::Add => make_op!(code, +),
            Opcode::Sub => make_op!(code, -),
            Opcode::Div => {
                if let Some(a) = code.stack.pop() {
                    if a == 0 {
                        Some(ProgramError::DivisionByZero)
                    } else if let Some(b) = code.stack.pop() {
                        code.stack.push(b / a);
                        None
                    } else {
                        Some(ProgramError::StackUnderflow)
                    }
                } else {
                    Some(ProgramError::StackUnderflow)
                }
            }
            Opcode::Done => break,
        } {
            return Err(err);
        }
    }

    if let Some(v) = code.stack.pop() {
        Ok(v)
    } else {
        Err(ProgramError::StackUnderflow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        use Opcode::*;
        assert_eq!(interpret(vec![Push(2), Done]).unwrap(), 2);
        assert_eq!(interpret(vec![Push(2), Push(3), Mul, Done]).unwrap(), 6);
        assert_eq!(interpret(vec![Push(5), Push(3), Sub, Done]).unwrap(), 2);
        assert!(interpret(vec![Push(2), Mul, Done]).is_err());
        assert!(interpret(vec![Push(2), Push(0), Div, Done]).is_err());
        assert_eq!(
            interpret(vec![
                Push(2),
                Push(2),
                Mul,
                Push(3),
                Mul,
                Push(4),
                Mul,
                Done
            ])
            .unwrap(),
            48
        );
        assert_eq!(
            interpret(vec![
                Push(5),
                Push(2),
                Mul,
                Push(5),
                Div,
                Push(2),
                Div,
                Done
            ])
            .unwrap(),
            1
        );
    }
}
