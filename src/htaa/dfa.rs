#[derive(Debug, Clone, PartialEq)]
enum State {
    Even,
    Odd,
}

struct DFA {
    current_state: State,
}

impl DFA {
    fn new() -> Self {
        DFA {
            current_state: State::Even,
        }
    }

    fn transition(&mut self, bit: char) -> Result<(), String> {
        match bit {
            '0' => {
                self.current_state = State::Even;
                Ok(())
            }
            '1' => {
                self.current_state = State::Odd;
                Ok(())
            }
            _ => Err(format!("Invald character '{}' only 0 and 1 allowed", bit)),
        }
    }

    fn process_string(&mut self, binary_str: &str) -> Result<bool, String> {
        if binary_str.is_empty() {
            return Ok(true);
        }

        self.current_state = State::Even;

        for ch in binary_str.chars() {
            self.transition(ch)?;
        }
        Ok(self.current_state == State::Even)
    }
}
