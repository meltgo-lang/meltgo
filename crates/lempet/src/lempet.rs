fn substring(s: &str, start: usize, length: usize) -> &str {
    if length == 0 {
        return "";
    }

    let mut ci = s.char_indices();
    let byte_start = match ci.nth(start) {
        Some(x) => x.0,
        None => return "",
    };

    match ci.nth(length - 1) {
        Some(x) => &s[byte_start..x.0],
        None => &s[byte_start..],
    }
}

enum LexFunction {
    F(Box<dyn Fn(String, &mut Vec<String>) -> Result<String, String>>),
}

impl LexFunction {
    pub fn run(
        &self,
        input: String,
        vec: &mut Vec<String>,
    ) -> Result<String, String> {
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
                Err(format!("'{}...' missmatch '{}'", substring(&input, 0, 4), s))
            }
        })));
        self
    }

    pub fn run(&mut self, input: &str) {
        let mut input = String::from(input);
        for item in self.vec.iter() {
            let result = item.run(input, &mut self.tokens);
            match result {
                Ok(res) => input = res,
                Err(msg) => {
                    println!("lexer error: {}", msg);
                    return ();
                }
            }
        }
    }

    pub fn get_tokens(&self) -> Vec<&str> {
        self.tokens
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
    }
}
