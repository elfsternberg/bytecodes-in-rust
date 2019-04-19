use std::result;

const REGISTER_COUNT: usize = 16;

pub enum Opcode {
    Load(usize, i64),
    Add(usize, usize, usize),
    Sub(usize, usize, usize),
    Mul(usize, usize, usize),
    Div(usize, usize, usize),
    Done(usize),
}

#[derive(Debug)]
pub enum ProgramError {
    DivisionByZero,
    UnexpectedTermination,
}

pub struct Program {
    program: Vec<Opcode>,
    registers: [i64; REGISTER_COUNT],
}

type Result<T> = result::Result<T, ProgramError>;

pub fn interpret(program: Vec<Opcode>) -> Result<i64> {
    let mut program = Program {
        program,
        registers: [0; REGISTER_COUNT],
    };

    for op in program.program {
        match op {
            Opcode::Load(r0, imm) => program.registers[r0] = imm,
            Opcode::Add(r0, r1, r2) => {
                program.registers[r2] = program.registers[r0] + program.registers[r1]
            }
            Opcode::Sub(r0, r1, r2) => {
                program.registers[r2] = program.registers[r0] - program.registers[r1]
            }
            Opcode::Mul(r0, r1, r2) => {
                program.registers[r2] = program.registers[r0] * program.registers[r1]
            }
            Opcode::Done(r0) => return Ok(program.registers[r0]),
            Opcode::Div(r0, r1, r2) => {
                if program.registers[r1] == 0 {
                    return Err(ProgramError::DivisionByZero);
                }
                program.registers[r2] = program.registers[r0] / program.registers[r1];
            }
        }
    }

    Err(ProgramError::UnexpectedTermination)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        use Opcode::*;
        assert_eq!(interpret(vec![Load(0, 2), Done(0)]).unwrap(), 2);
        assert_eq!(
            interpret(vec![Load(1, 2), Load(2, 3), Mul(1, 2, 0), Done(0)]).unwrap(),
            6
        );
        assert_eq!(
            interpret(vec![
                Load(1, 2),
                Load(2, 2),
                Mul(1, 2, 0),
                Load(1, 3),
                Mul(1, 0, 0),
                Load(1, 4),
                Mul(1, 0, 0),
                Done(0)
            ])
            .unwrap(),
            48
        );
    }
}
