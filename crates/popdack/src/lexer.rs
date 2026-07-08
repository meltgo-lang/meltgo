pub enum LexReturn<'a> {
    Some(&'a str),
    Array(Vec<&'a str>),
}

pub type LexResult<'a> = Option<(LexReturn<'a>, &'a str)>;

pub trait LexRule<'a> {
    fn lparse(&mut self, input: &str) -> LexResult<'a>;
}

pub struct Lexer<'a> {
    rules: Vec<Box<dyn LexRule<'a>>>,
    tokens: Vec<&'a str>,
    rule_pos: Vec<(&'a Box<dyn LexRule<'a> + 'static>, usize)>,
}

impl<'a> Lexer<'a> {
    pub fn new() -> Self {
        Self {
            rules: vec![],
            tokens: vec![],
            rule_pos: vec![],
        }
    }

    pub fn add_rule(&mut self, rule: impl LexRule<'a> + Clone + 'static) {
        self.rules.push(Box::new(rule));
    }

    fn excute(&'a mut self, input: &'a str) -> (bool, &'a str) {
        let mut input = input;
        let mut tf = false;
        let index = self.tokens.len();
        for rule in self.rules.iter_mut() {
            match rule.lparse(input) {
                Some((result, rest)) => {
                    input = rest;
                    tf = true;
                    self.rule_pos.push((rule, index));
                    match result {
                        LexReturn::Some(w) => {
                            self.tokens.push(w);
                        },
                        LexReturn::Array(vec) => {
                            for item in vec.iter() {
                                self.tokens.push(item);
                            }
                        }
                    }
                    break;
                },
                None => (),
            }
        }
        (tf, input)
    }

    pub fn run(&mut self) {}
}
