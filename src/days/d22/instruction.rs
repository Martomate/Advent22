#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Move(u32),
    TurnLeft,
    TurnRight,
}

pub fn parse_instructions(s: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut steps: u32 = 0;

    for c in s.chars() {
        if c == 'L' {
            instructions.push(Instruction::Move(steps));
            instructions.push(Instruction::TurnLeft);
            steps = 0;
        } else if c == 'R' {
            instructions.push(Instruction::Move(steps));
            instructions.push(Instruction::TurnRight);
            steps = 0;
        } else {
            steps = steps * 10 + ((c as u8 - b'0') as u32);
        }
    }

    instructions.push(Instruction::Move(steps));

    instructions
        .into_iter()
        .filter(|i| match *i {
            Instruction::Move(n) => n != 0,
            _ => true,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instructions_handles_one_number() {
        assert_eq!(parse_instructions("123"), vec![Instruction::Move(123)]);
    }

    #[test]
    fn parse_instructions_handles_one_turn() {
        assert_eq!(parse_instructions("L"), vec![Instruction::TurnLeft]);
        assert_eq!(parse_instructions("R"), vec![Instruction::TurnRight]);
    }

    #[test]
    fn parse_instructions_handles_one_number_and_one_turn() {
        assert_eq!(
            parse_instructions("123L"),
            vec![Instruction::Move(123), Instruction::TurnLeft]
        );
        assert_eq!(
            parse_instructions("L123"),
            vec![Instruction::TurnLeft, Instruction::Move(123)]
        );
    }

    #[test]
    fn parse_instructions_handles_simple_example() {
        use Instruction::*;

        assert_eq!(
            parse_instructions("10R5L5R10L4R5L5"),
            vec![
                Move(10),
                TurnRight,
                Move(5),
                TurnLeft,
                Move(5),
                TurnRight,
                Move(10),
                TurnLeft,
                Move(4),
                TurnRight,
                Move(5),
                TurnLeft,
                Move(5)
            ]
        );
    }
}
