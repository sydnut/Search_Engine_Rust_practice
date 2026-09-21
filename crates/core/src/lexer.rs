use snowstem::{Algorithm, Stemmer};
use std::sync::LazyLock;

#[derive(Debug)]
pub struct Lexer<'a> {
    content: &'a [char],
    stemmer: &'static Stemmer,
}
static SNOW_STEMMER: LazyLock<Stemmer> = LazyLock::new(|| Stemmer::create(Algorithm::English));

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self {
            content,
            stemmer: &SNOW_STEMMER,
        }
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
    fn next_token(&mut self) -> Option<String> {
        self.trim_left();
        if self.content.is_empty() {
            return None;
        }
        if self.content[0].is_alphabetic() {
            Some(
                self._yield_while(|c| c.is_alphanumeric())
                    .iter()
                    .map(|c| c.to_ascii_lowercase())
                    .collect(),
            )
        } else if self.content[0].is_numeric() {
            Some(self._yield_while(|c| c.is_numeric()).iter().collect())
        } else {
            Some(self._yield(1).iter().collect())
        }
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.next_token();
        if res.is_none() {
            None
        } else {
            let token = res.unwrap();
            let cow_str = self.stemmer.stem(&token);
            Some(cow_str.into_owned())
        }
    }
}
