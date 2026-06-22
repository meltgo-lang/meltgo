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
    Call(&'a mut LexerBuilder<'a>),
    Predicate(Box<dyn Fn(char) -> bool>),
    EOF,
}

struct Lexer {
    tokens: Vec<String>,
    err_flg: bool,
    add: usize,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            tokens: vec![],
            err_flg: false,
            add: 0,
        }
    }

    pub fn word(&mut self, input: String, w: &str, idx: usize) -> Result<(String, usize), String> {
        let s = String::from(w);
        if input.starts_with(&s) {
            self.tokens[idx] = s.clone();
            Ok((
                match input.char_indices().nth(s.len()) {
                    Some((ch_idx, _)) => String::from(&input[ch_idx..]),
                    None => String::new(),
                },
                idx + 1,
            ))
        } else {
            self.err_flg = true;
            Err(format!(
                "'{}...' missmatch '{}'",
                substring(&input, 0, s.char_indices().count()),
                s
            ))
        }
    }

    pub fn repeat_str(
        &mut self,
        input: String,
        w: &str,
        idx: usize,
    ) -> Result<(String, usize), String> {
        let s = String::from(w);
        if input.starts_with(&s) {
            self.tokens[idx] = self.tokens[idx].clone() + &s;
            Ok((
                match input.char_indices().nth(s.len()) {
                    Some((ch_idx, _)) => String::from(&input[ch_idx..]),
                    None => String::new(),
                },
                idx,
            ))
        } else if input.is_empty() {
            self.err_flg = true;
            Err("empty input".to_string())
        } else {
            self.err_flg = true;
            Err(format!(
                "'{}...' missmatch '{}'",
                substring(&input, 0, s.char_indices().count()),
                s
            ))
        }
    }

    pub fn err_jmp(
        &mut self,
        input: String,
        w: char,
        idx: usize,
    ) -> Result<(String, usize), String> {
        if self.err_flg {
            let vec = input.char_indices().collect::<Vec<(usize, char)>>();
            match vec.get(1) {
                Some((ch_idx, _)) => {
                    if vec[0].1 == w {
                        self.err_flg = false;
                        Ok((input, idx + 1))
                    } else {
                        Ok((String::from(&input[*ch_idx..]), idx))
                    }
                }
                None => {
                    self.err_flg = true;
                    Err("empty input".to_string())},
            }
        } else {
            Ok((input, idx + 1))
        }
    }

    pub fn call<'a>(&mut self, input: String, lb: &mut LexerBuilder<'a>, idx: usize) -> Result<(String, usize), String> {
        let _ = lb.run(input.as_str());
        if lb.lexer.err_flg {
            self.err_flg = true;
            Err(String::from("failure to call"))
        }
        else {
            let tokens = lb.get_tokens();
            for item in tokens.clone() {
                self.tokens.push((*item).to_string());
            }
            self.add = tokens.len();
            Ok((lb.rest.clone().unwrap(), idx + 1))
        }
    }

    pub fn predicate(
        &mut self,
        input: String,
        f: impl Fn(char) -> bool,
        idx: usize,
    ) -> Result<(String, usize), String> {
        let vec = input.char_indices().collect::<Vec<(usize, char)>>();
        match vec.get(0) {
            Some((ch_idx, ch)) => {
                if f(*ch) {
                    self.tokens[idx] = self.tokens[idx].clone() + &ch.to_string();
                    Ok((String::from(&input[ch_idx + ch.len_utf8()..]), idx+1))
                } else {
                    self.err_flg = true;
                    Err(format!(
                "'{}...' missmatch predicate",
                substring(&input, 0, input.char_indices().count()),
            ))
                }
            }
            None => {
                self.err_flg = true;
                Err("empty input".to_string())},
        }
    }

    pub fn eof(&mut self, input: String, idx: usize) -> Result<(String, usize), String> {
        if input.is_empty() {
            Ok((input, idx +1))
        }
        else {
            self.err_flg = true;
            Err(String::from("it is not end of file"))
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
    rest: Option<String>,
}

impl<'a> LexerBuilder<'a> {
    pub fn new() -> Self {
        Self {
            vec: vec![],
            lexer: Lexer::new(),
            rest: None,
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

    pub fn call(&mut self, lb: &'a mut LexerBuilder<'a>) -> &mut Self {
        self.vec.push(LexerFunction::Call(lb));
        self
    }

    pub fn predicate(&mut self, f: impl Fn(char) -> bool + 'static) -> &mut Self {
        self.vec.push(LexerFunction::Predicate(Box::new(f)));
        self
    }

    pub fn eof(&mut self) -> &mut Self {
        self.vec.push(LexerFunction::EOF);
        self
    }

    pub fn run(&mut self, input: &str) -> &mut Self {
        let mut input = String::from(input);
        let mut idx: usize = 0;
        let mut val_idx: usize = 0;
        while val_idx < self.vec.len() {
            if idx == self.lexer.tokens.len() {
                self.lexer.tokens.push(String::new());
            }
            let result = match self.vec[val_idx] {
                LexerFunction::Word(w) => self.lexer.word(input.clone(), w, idx),
                LexerFunction::Repeat(w) => self.lexer.repeat_str(input.clone(), w, idx),
                LexerFunction::ErrJmp(w) => self.lexer.err_jmp(input.clone(), w, idx),
                LexerFunction::EOF => self.lexer.eof(input.clone(), idx),
                _ => {
                    if let LexerFunction::Call(lb) = &mut self.vec[idx] {
                        self.lexer.call(input.clone(), lb, idx)
                    }
                    else if let LexerFunction::Predicate(f) = &self.vec[idx] {
                        self.lexer.predicate(input.clone(), f, idx)
                    }
                    else {
                        Err(String::from("pattern not found"))
                    }
                }
            };
            match result {
                Ok((new_input, new_idx)) => {
                    input = new_input;
                    idx = new_idx;
                    val_idx = new_idx;
                }
                Err(msg) => {
                    println!("{}", msg);
                    idx += 1;
                    val_idx += 1;
                    println!("({}, {})", idx, val_idx);
                }
            }
            idx += self.lexer.add;
            self.lexer.add = 0;
        }
        self.rest = Some(input);
        self
    }

    pub fn get_tokens(&self) -> Vec<&str> {
        self.lexer.get_tokens()
    }
}
