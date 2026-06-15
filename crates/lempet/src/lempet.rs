enum LexFunction {
    F(Box<dyn Fn(String, &mut Vec<String>) -> Result<String, &str>>),
}

impl LexFunction {
    pub fn unwrap<'a>(
        &'a self,
        input: String,
        vec: &'a mut Vec<String>,
    ) -> Result<String, &'a str> {
        match &self {
            Self::F(f) => f(input, vec),
            _ => panic!(""),
        }
    }
}

pub struct Lexer {
    vec: Vec<LexFunction>,
    tokens: Vec<String>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            vec: vec![],
            tokens: vec![],
        }
    }

    pub fn word(&mut self, w: &str) -> &mut Lexer {
        let s = String::from(w);
        self.vec.push(LexFunction::F(Box::new(move |input, result| {
            if input.starts_with(&s) {
                result.push(s.clone());
                Ok(match input.char_indices().nth(s.len()) {
                    Some((idx, _)) => String::from(&input[idx..]),
                    None => String::new(),
                })
            } else {
                Err("")
            }
        })));
        self
    }

    pub fn excute(&mut self, input: &str) {
        let mut input = String::from(input);
        for item in self.vec.iter() {
            input = item.unwrap(input, &mut self.tokens).unwrap();
        }
    }

    pub fn get_tokens(&self) -> Vec<&str> {
        self.tokens
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
    }
}
