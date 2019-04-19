pub enum Opcode {
    Inc,
    Dec,
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
            interpret(vec![Inc, Inc, Inc, Dec, Dec, Done, Inc, Inc]),
            Some(1)
        );
    }

}
