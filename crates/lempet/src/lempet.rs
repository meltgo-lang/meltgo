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

enum LexerFunction<'a> {
    Word(&'a str),
    Repeat(&'a str),
    ErrJmp(char),
}

struct Lexer {
    tokens: Vec<String>,
    err_flg: bool,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            tokens: vec![],
            err_flg: false,
        }
    }

    pub fn word(&mut self, input: String, w: &str, idx: usize) -> Result<(String, usize), String> {
        let s = String::from(w);
        if input.starts_with(&s) {
            self.tokens[idx] = s.clone();
            Ok((match input.char_indices().nth(s.len()) {
                Some((ch_idx, _)) => String::from(&input[ch_idx..]),
                None => String::new(),
            }, idx + 1))
        } else {
            self.err_flg = true;
            Err(format!("'{}...' missmatch '{}'", substring(&input, 0, s.char_indices().count()), s))
        }
    }

    pub fn repeat(&mut self, input: String, w: &str, idx: usize) -> Result<(String, usize), String> {
        let s = String::from(w);
        if input.starts_with(&s) {
            self.tokens[idx] = self.tokens[idx].clone() + &s;
            Ok((match input.char_indices().nth(s.len()) {
                Some((ch_idx, _)) => String::from(&input[ch_idx..]),
                None => String::new(),
            }, idx))
        }
        else if input.is_empty() {
            Err("empty input".to_string())
        }
        else {
            Err(format!("'{}...' missmatch '{}'", substring(&input, 0, s.char_indices().count()), s))
        }
    }

    pub fn err_jmp(&mut self, input: String, w: char, idx: usize) -> Result<(String, usize), String> {
        if self.err_flg {
            let vec = input.char_indices().collect::<Vec<(usize, char)>>();
            match vec.get(1) {
                Some((ch_idx, _)) => {
                    if vec[0].1 == w {
                        self.err_flg = false;
                        Ok((input, idx + 1))
                    }
                    else {
                        Ok((String::from(&input[*ch_idx..]), idx))
                    }
                },
                None => Err("empty input".to_string()),
            }
        }
        else {
            Ok((input, idx + 1))
        }
    }

    pub fn get_tokens(&self) -> Vec<&str> {
        self.tokens
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
    }
}

pub struct LexerBuilder<'a> {
    vec: Vec<LexerFunction<'a>>,
    lexer: Lexer,
}

impl<'a> LexerBuilder<'a> {
    pub fn new() -> Self {
        Self {
            vec: vec![],
            lexer: Lexer::new(),
        }
    }

    pub fn word(&mut self, w: &'a str) -> &mut Self {
        self.vec.push(LexerFunction::Word(w));
        self
    }

    pub fn repeat(&mut self, w: &'a str) -> &mut Self {
        self.vec.push(LexerFunction::Repeat(w));
        self
    }

    pub fn err_jmp(&mut self, w: char) -> &mut Self {
        self.vec.push(LexerFunction::ErrJmp(w));
        self
    }

    pub fn run(&mut self, input: &str) {
        let mut input = String::from(input);
        let mut idx: usize = 0;
        while idx < self.vec.len() {
            if idx == self.lexer.tokens.len() {
                self.lexer.tokens.push(String::new());
            }
            let result =
                match self.vec[idx] {
                    LexerFunction::Word(w) => self.lexer.word(input.clone(), w, idx),
                    LexerFunction::Repeat(w) => self.lexer.repeat(input.clone(), w, idx),
                    LexerFunction::ErrJmp(w) => self.lexer.err_jmp(input.clone(), w, idx),
                };
            match result {
                Ok((new_input, new_idx)) => {
                    input = new_input;
                    idx = new_idx;
                },
                Err(msg) => {
                    println!("{}", msg);
                    idx += 1;
                }
            }
        }
    }

    pub fn get_tokens(&self) -> Vec<&str> {
        self.lexer.get_tokens()
    }
}
