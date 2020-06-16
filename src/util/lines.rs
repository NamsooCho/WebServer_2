pub struct Lines {
    src: Vec<u8>,
    cursor: usize,
}

impl Lines {
    pub fn new(src: Vec<u8>) -> Lines {
        Lines {
            src,
            cursor: 0,
        }
    }
}

impl Iterator for Lines {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let from = self.cursor;
        let mut to = from;

        loop {
            if self.cursor >= self.src.len() {
                break;
            }

            let c = self.src[self.cursor];

            if c == b'\n' {
                if self.src[self.cursor - 1] == b'\r' {
                    to = self.cursor - 1;
                } else {
                    to = self.cursor;
                }

                self.cursor += 1;
                break;
            }

            self.cursor += 1;
        }

        if from >= to {
            None
        } else {
            Some(String::from_utf8_lossy(&self.src[from..to]).to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let src = "hello\r\nworld\n\r".as_bytes().to_vec();
        let mut lines = Lines::new(src);

        let result = lines.next();
        if let Some(line) = result {
            assert_eq!(line, "hello");
        } else {
            assert!(false);
        }

        let result = lines.next();
        if let Some(line) = result {
            assert_eq!(line, "world");
        } else {
            assert!(false);
        }

        assert_eq!(lines.next(), None);
    }
}
