pub struct CommandHistory {
    commands: Vec<String>,
    limit: usize,
}

impl CommandHistory {
    pub fn new(limit: usize) -> CommandHistory {
        CommandHistory {
            commands: Vec::with_capacity(limit),
            limit,
        }
    }

    pub fn add(&mut self, command: String) {
        if self.commands.len() >= self.limit {
            self.commands.remove(0);
        }
        self.commands.push(command);
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.commands
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut history = CommandHistory::new(3);
        history.add("ls".to_string());
        history.add("cd".to_string());
        history.add("pwd".to_string());
        history.add("cat".to_string());
        assert_eq!(history.get_history(), &vec!["cd", "pwd", "cat"]);
    }
}
