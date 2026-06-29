use std::sync::{Arc, Mutex, MutexGuard};

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

pub type Predicate = dyn Fn(char) -> bool;

pub struct LexerFunctionsBuffer<T: ?Sized> {
    buf: Vec<Box<T>>,
}

impl<T: ?Sized> LexerFunctionsBuffer<T> {
    pub fn new() -> Self {
        Self { buf: vec![] }
    }
}

#[derive(Clone, Debug)]
enum LexerFunction<'a> {
    Word(&'a str),
    Repeat(&'a str),
    ErrJmp(&'a str),
    Call(Arc<Mutex<LexerBuilder<'a>>>),
    RepeatCall(Arc<Mutex<LexerBuilder<'a>>>),
    Predicate(usize),
    EOF,
}

#[derive(Clone, Debug)]
enum LexerResult<'a> {
    Str(String),
    LB(Arc<Mutex<LexerBuilder<'a>>>),
    None,
}

#[derive(Clone, Debug)]
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

    pub fn word<'a>(
        &mut self,
        input: String,
        w: &str,
        idx: usize,
    ) -> Result<(LexerResult<'a>, usize), String> {
        let s = String::from(w);
        if input.starts_with(&s) {
            self.tokens[idx] += &s.clone();
            Ok((
                match input.char_indices().nth(s.len()) {
                    Some((ch_idx, _)) => LexerResult::Str(String::from(&input[ch_idx..])),
                    None => LexerResult::Str(input),
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

    pub fn repeat_str<'a>(
        &mut self,
        input: String,
        w: &str,
        idx: usize,
    ) -> Result<(LexerResult<'a>, usize), String> {
        let s = String::from(w);
        if input.starts_with(&s) {
            self.tokens[idx] += &s;
            Ok((
                match input.char_indices().nth(s.len()) {
                    Some((ch_idx, _)) => LexerResult::Str(String::from(&input[ch_idx..])),
                    None => LexerResult::Str(String::new()),
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

    pub fn repeat<'a>(
        &mut self,
        input: String,
        lb: Arc<Mutex<LexerBuilder<'a>>>,
        buf: &mut LexerFunctionsBuffer<Predicate>,
        idx: usize,
    ) -> Result<(LexerResult<'a>, usize), String> {
        let mut shared_lb = lb.lock().unwrap();
        let _lb = shared_lb.run(input.as_str(), buf);
        if shared_lb.lexer.err_flg {
            Ok((LexerResult::LB(Arc::clone(&lb)), idx + 1))
        } else {
            Ok((LexerResult::LB(Arc::clone(&lb)), idx))
        }
    }

    pub fn err_jmp<'a>(
        &mut self,
        input: String,
        w: &str,
        idx: usize,
    ) -> Result<(LexerResult<'a>, usize), String> {
        if self.err_flg {
            let vec = input.char_indices().collect::<Vec<(usize, char)>>();
            match vec.get(1) {
                Some((ch_idx, _)) => {
                    if input.starts_with(w) {
                        self.err_flg = false;
                        Ok((LexerResult::Str(input), idx + 1))
                    } else {
                        Ok((LexerResult::Str(String::from(&input[*ch_idx..])), idx))
                    }
                }
                None => {
                    self.err_flg = true;
                    Err("empty input".to_string())
                }
            }
        } else {
            Ok((LexerResult::Str(input), idx + 1))
        }
    }

    pub fn call<'a>(
        &mut self,
        input: String,
        lb: Arc<Mutex<LexerBuilder<'a>>>,
        buf: &mut LexerFunctionsBuffer<Predicate>,
        idx: usize,
    ) -> Result<(LexerResult<'a>, usize), String> {
        let mut shared_lb = lb.lock().unwrap();
        let _lb = shared_lb.run(input.as_str(), buf);
        if shared_lb.lexer.err_flg {
            self.err_flg = true;
            Err(String::from("failure to call"))
        } else {
            Ok((LexerResult::LB(Arc::clone(&lb)), idx + 1))
        }
    }

    pub fn predicate<'a>(
        &mut self,
        input: String,
        buf: &LexerFunctionsBuffer<Predicate>,
        f: usize,
        idx: usize,
    ) -> Result<(LexerResult<'a>, usize), String> {
        let vec = input.char_indices().collect::<Vec<(usize, char)>>();
        match vec.get(0) {
            Some((ch_idx, ch)) => {
                if buf.buf[f](*ch) {
                    self.tokens[idx] = self.tokens[idx].clone() + &ch.to_string();
                    Ok((
                        LexerResult::Str(String::from(&input[ch_idx + ch.len_utf8()..])),
                        idx + 1,
                    ))
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
                Err("empty input".to_string())
            }
        }
    }

    pub fn eof<'a>(
        &mut self,
        input: String,
        idx: usize,
    ) -> Result<(LexerResult<'a>, usize), String> {
        if input.is_empty() {
            Ok((LexerResult::Str(input), idx + 1))
        } else {
            self.err_flg = true;
            Err(String::from("it is not end of file"))
        }
    }
}

#[derive(Clone, Debug)]
pub struct LexerBuilder<'a> {
    vec: Vec<LexerFunction<'a>>,
    lb_point: Vec<(usize, Arc<Mutex<LexerBuilder<'a>>>)>,
    lexer: Lexer,
    rest: Option<String>,
}

impl<'a> LexerBuilder<'a> {
    pub fn new() -> Self {
        Self {
            vec: vec![],
            lb_point: vec![],
            lexer: Lexer::new(),
            rest: None,
        }
    }

    pub fn word(mut self, w: &'a str) -> Self {
        self.vec.push(LexerFunction::Word(w));
        self
    }

    pub fn repeat_str(mut self, w: &'a str) -> Self {
        self.vec.push(LexerFunction::Repeat(w));
        self
    }

    pub fn repeat(mut self, lb: LexerBuilder<'a>) -> Self {
        self.vec
            .push(LexerFunction::RepeatCall(Arc::new(Mutex::new(lb))));
        self
    }

    pub fn err_jmp(mut self, w: &'a str) -> Self {
        self.vec.push(LexerFunction::ErrJmp(w));
        self
    }

    pub fn call(mut self, lb: LexerBuilder<'a>) -> Self {
        self.vec.push(LexerFunction::Call(Arc::new(Mutex::new(lb))));
        self
    }

    pub fn predicate(
        mut self,
        buf: &mut LexerFunctionsBuffer<Predicate>,
        f: Box<Predicate>,
    ) -> Self {
        let l = buf.buf.len();
        buf.buf.push(f);
        self.vec.push(LexerFunction::Predicate(l));
        self
    }

    pub fn eof(mut self) -> Self {
        self.vec.push(LexerFunction::EOF);
        self
    }

    pub fn run(&mut self, input: &str, buf: &mut LexerFunctionsBuffer<Predicate>) -> &mut Self {
        let mut input = String::from(input);
        let mut idx: usize = 0;
        while idx < self.vec.len() {
            let tokens = &mut self.lexer.tokens;
            if idx == tokens.len() {
                tokens.push(String::new());
            }

            let result = {
                match &self.vec[idx] {
                    LexerFunction::Word(w) => self.lexer.word(input.clone(), w, idx),
                    LexerFunction::Repeat(w) => self.lexer.repeat_str(input.clone(), w, idx),
                    LexerFunction::ErrJmp(w) => self.lexer.err_jmp(input.clone(), w, idx),
                    LexerFunction::EOF => self.lexer.eof(input.clone(), idx),
                    LexerFunction::Predicate(f) => {
                        self.lexer.predicate(input.clone(), buf, *f, idx)
                    }
                    LexerFunction::Call(lb) => {
                        self.lexer.call(input.clone(), Arc::clone(lb), buf, idx)
                    }
                    LexerFunction::RepeatCall(lb) => {
                        self.lexer.repeat(input.clone(), Arc::clone(lb), buf, idx)
                    }
                }
            };
            match result {
                Ok((new_input, new_idx)) => {
                    input = match new_input {
                        LexerResult::Str(s) => s.to_string(),
                        LexerResult::LB(lb) => {
                            let cloned_lb = Arc::clone(&lb);
                            self.lb_point.push((idx, cloned_lb));
                            let cloned_lb = Arc::clone(&lb);
                            let shared_lb = cloned_lb.lock().unwrap();
                            shared_lb.rest.as_ref().unwrap().clone()
                        }
                        LexerResult::None => input,
                    };
                    idx = new_idx;
                }
                Err(msg) => {
                    println!("{}", msg);
                    idx += 1;
                }
            }
        }
        self.rest = Some(input.clone());
        self
    }

    pub fn set_tokens(&mut self) -> () {
        for item in self.lb_point.iter() {
            let (idx, lb) = item;
            let mut idx = *idx;
            self.lexer.tokens.remove(idx);
            let shared_lb = lb.lock().unwrap();
            for item2 in shared_lb.lexer.tokens.iter() {
                self.lexer.tokens.insert(idx, item2.to_string());
                idx += 1;
            }
        }
    }
    pub fn get_tokens(&self) -> Vec<&str> {
        self.lexer
            .tokens
            .iter()
            .map(|x| x.as_str())
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
    }
}
