pub mod polymer {
    const ASCII_DIFF: i32 = 0x20;
    pub struct Reader {
        stack: Vec<u8>,
        head: usize,
    }

    impl Reader {
        pub fn new() -> Self {
            Reader {
                stack: Vec::with_capacity(35_000),
                head: 0,
            }
        }

        fn peek(&self) -> Option<&u8> {
            if self.stack.len() > 0 {
                Some(&(self.stack[self.head]))
            } else {
                None
            }
        }

        pub fn try_read(&mut self, unit: u8) {
            if let Some(&u) = self.peek() {
                if (u as i32 - unit as i32).abs() == ASCII_DIFF {
                    if self.stack.len() > 1 {
                        self.head -= 1;
                    }
                    self.stack.pop().unwrap();
                } else {
                    self.stack.push(unit);
                    self.head += 1;
                }
            } else {
                self.stack.push(unit);
            }
        }

        pub fn len(&self) -> usize {
            self.stack.len()
        }

        pub fn debug(self) {
            let s = String::from_utf8(self.stack).expect("Bummer");
            println!("{}", s);
            println!("{}", s.len());
        }

        pub fn stringify(self) -> String {
            String::from_utf8(self.stack).expect("Bummer")
        }
    }
}
