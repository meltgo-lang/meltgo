use lempet::lex::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MyTokens {
    Number,
    Operator(String),
    EOF,
}

struct NumberRule;
impl LexRule<MyTokens> for NumberRule {
    fn matches(&self, c: char) -> bool { c.is_ascii_digit() }
    fn consume(&self, cursor: &mut Cursor<'_>) -> Result<(MyTokens, String), String> {
        while let Some(&c) = cursor.peek() {
            if c.is_ascii_digit() { cursor.advance(); } else { break; }
        }
        Ok((MyTokens::Number, String::new()))
    }
}

struct OpRule;
impl LexRule<MyTokens> for OpRule {
    fn matches(&self, c: char) -> bool { "+-=>&|".contains(c) }
    fn consume(&self, cursor: &mut Cursor<'_>) -> Result<(MyTokens, String), String> {
        let first = cursor.advance().unwrap();
        let mut op = first.to_string();
        
        if let Some(&second) = cursor.peek() {
            if (first == '=' && second == '=') || (first == '=' && second == '>') {
                op.push(cursor.advance().unwrap());
            }
        }
        Ok((MyTokens::Operator(op), String::new()))
    }
}

pub fn lt() {
    let mut lexer = Lexer::new("42 == 100 =>", MyTokens::EOF);
    
    lexer.add_rule(NumberRule);
    lexer.add_rule(OpRule);

    loop {
        match lexer.next_token() {
            Ok(token) => {
                println!("{:?}", token);
                if token.kind == MyTokens::EOF { break; }
            }
            Err(e) => { println!("Error: {}", e); break; }
        }
    }
}