pub enum Opcode {
    Inc,
    Dec,
    Add(i64), // New
    Sub(i64), // New
    Done,
}

pub struct Program {
    program: Vec<Opcode>,
    accumulate: i64,
}

pub fn interpret(program: Vec<Opcode>) -> Option<i64> {
    let mut code = Program {
        program,
        accumulate: 0,
    };

    for i in code.program {
        code.accumulate = match i {
            Opcode::Inc => code.accumulate + 1,
            Opcode::Dec => code.accumulate - 1,
            Opcode::Add(i) => code.accumulate + i, // New
            Opcode::Sub(i) => code.accumulate - i, // New
            Opcode::Done => break,
        }
    }

    Some(code.accumulate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longer() {
        use Opcode::*;
        assert_eq!(
            interpret(vec![Inc, Inc, Inc, Add(4), Add(3), Sub(6), Done]),
            Some(4)
        );
    }

}
