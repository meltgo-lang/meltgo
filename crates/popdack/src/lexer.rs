pub enum LexReturn {
    Some(String),
    Array(Vec<String>),
    None,
}

pub type LexResult = Option<(LexReturn, String)>;

pub trait LexRule: LexClone {
    fn lparse(&mut self, input: &mut String) -> LexResult;
}

pub trait LexClone {
    fn clone_box(&self) -> Box<dyn LexRule>;
}

impl Clone for Box<dyn LexRule> {
    fn clone(&self) -> Box<dyn LexRule> {
        self.clone_box()
    }
}

pub struct Lexer {
    rules: Vec<Box<dyn LexRule>>,
    tokens: Vec<String>,
    rule_pos: Vec<(Box<dyn LexRule + 'static>, usize)>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            rules: vec![],
            tokens: vec![],
            rule_pos: vec![],
        }
    }

    pub fn add_rule(&mut self, rule: impl LexRule + Clone + 'static) {
        self.rules.push(Box::new(rule));
    }

    fn excute(&mut self, input: &mut String) -> LexReturn {
        let index = self.tokens.len();
        for rule in self.rules.iter_mut() {
            let lparsed = rule.lparse(input);
            match lparsed {
                Some((r, _)) => {
                    self.rule_pos.push((rule.clone(), index));
                    return r;
                }
                None => (),
            }
        }
        LexReturn::None
    }

    pub fn run(&mut self, input: &str) {
        let mut input = input.to_string();
        while !input.is_empty() {
            let res = self.excute(&mut input);
            match res {
                LexReturn::Some(w) => self.tokens.push(w),
                LexReturn::Array(vec) => {
                    for item in vec.iter() {
                        self.tokens.push(item.clone());
                    }
                }
                LexReturn::None => break,
            }
        }
    }

    pub fn get_tokens(&self) -> Vec<&str> {
        self.tokens
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
    }
}
