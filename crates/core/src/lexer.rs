#[derive(Debug)]
pub struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }
    fn trim_left(&mut self) {
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }
    fn _yield(&mut self, n: usize) -> &'a [char] {
        let res = &self.content[0..n];
        self.content = &self.content[n..];
        res
    }
    fn _yield_while(&mut self, mut predicate: impl FnMut(&char) -> bool) -> &'a [char] {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }
        self._yield(n)
    }
    pub fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();
        if self.content.is_empty() {
            return None;
        }
        if self.content[0].is_alphabetic() {
            Some(self._yield_while(|c| c.is_alphanumeric()))
        } else if self.content[0].is_numeric() {
            Some(self._yield_while(|c| c.is_numeric()))
        } else {
            Some(self._yield(1))
        }
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
