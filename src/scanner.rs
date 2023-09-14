pub struct Scanner {
    cursor: usize,
    characters: Vec<char>,
}

impl Scanner {
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect(),
        }
    }
    pub fn cursor(&self) -> usize {
        return self.cursor;
    }

    pub fn is_done(&self) -> bool {
        return self.cursor == self.characters.len();
    }

    pub fn transform<T>(&mut self, cb: impl FnOnce(&char) -> Option<T>) -> Option<T> {
        match self.characters.get(self.cursor) {
            Some(input) => match cb(input) {
                Some(output) => {
                    self.cursor += 1;
                    return Some(output);
                }
                None => None,
            },
            None => None,
        }
    }

    pub fn take(&mut self, target: &char) -> bool {
        match self.characters.get(self.cursor()) {
            Some(char) => {
                if target == char {
                    self.cursor += 1;
                    return true;
                } else {
                    return false;
                }
            }
            None => false,
        }
    }
}
