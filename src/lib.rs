// use std::collections::HashMap;

pub trait Tokenizable {
    fn new() -> Self;
    fn encode(&mut self, input: &str) -> ();
    fn decode(&self) -> ();
}

pub struct Tokenizer {
    input: Vec<u8>,
    // token_map: HashMap<String, String>,
}

impl Tokenizable for Tokenizer {
    fn new() -> Tokenizer {
        Tokenizer {
            input: Vec::new(),
            // token_map: HashMap::new(),
        }
    }

    fn encode(&mut self, input: &str) -> () {
        self.input = input.as_bytes().to_vec();
    }

    fn decode(&self) -> () {
        // decode the token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tokenizer = Tokenizer::new();
        tokenizer.encode("test");
        assert_eq!(tokenizer.input, b"test");
    }
}
