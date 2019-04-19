pub enum Opcode {
    Chr(char),
    Jmp(isize),
    Alt(isize, isize),
    End,
}

pub fn interpret_reentrant(
    program: &[Opcode],
    sample: &[char],
    mut pc: isize,
    mut sp: isize,
) -> bool {
    loop {
        if sp < 0 || (sp as usize) > sample.len() || pc < 0 || (pc as usize) > program.len() {
            return false;
        }

        match program[pc as usize] {
            Opcode::Chr(c) => {
                if c == sample[sp as usize] {
                    pc += 1;
                    sp += 1;
                } else {
                    return false;
                }
            }

            Opcode::Jmp(i) => {
                pc += i;
            }

            Opcode::Alt(l, r) => {
                if interpret_reentrant(program, sample, pc + l, sp) {
                    return true;
                }
                pc += r;
            }

            Opcode::End => return true,
        }
    }
}

pub fn interpret(program: Vec<Opcode>, sample: &str) -> bool {
    let s: Vec<char> = sample.chars().collect();
    interpret_reentrant(&program, &s, 0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        use Opcode::*;
        // "a"
        assert!(interpret(vec![Chr('a'), End], "a"));
    }

    #[test]
    fn seq() {
        use Opcode::*;
        // "ab"
        assert!(interpret(vec![Chr('a'), Chr('b'), End], "ab"));
    }

    #[test]
    fn alt() {
        use Opcode::*;
        // "(a|b)c"
        //
        // This sample includes a particularly silly example of poor
        // optimization: `Jmp(1)` is basically a no-op that does no
        // processing other than to increment the pc up one.
        assert!(interpret(
            vec![Alt(1, 3), Chr('a'), Jmp(2), Chr('b'), Jmp(1), Chr('c'), End],
            "ac"
        ));
        assert!(interpret(
            vec![Alt(1, 3), Chr('a'), Jmp(2), Chr('b'), Jmp(1), Chr('c'), End],
            "bc"
        ));
        assert_eq!(
            interpret(
                vec![Alt(1, 3), Chr('a'), Jmp(2), Chr('b'), Jmp(1), Chr('c'), End],
                "ab"
            ),
            false
        );
    }
}
