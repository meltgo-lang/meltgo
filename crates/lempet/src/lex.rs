use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token<K> {
    pub kind: K,
    pub span: Span,
    pub literal: String,
}

pub struct Cursor<'a> {
    input: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            line: 1,
            column: 1,
        }
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    pub fn peek_second(&mut self) -> Option<char> {
        let mut iter = self.input.clone();
        iter.next();
        iter.next()
    }

    pub fn advance(&mut self) -> Option<char> {
        let ch = self.input.next();
        if let Some(c) = ch {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        ch
    }

    pub fn current_span(&self) -> Span {
        Span {
            line: self.line,
            column: self.column,
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
}

pub trait LexRule<K> {
    fn matches(&self, next_char: char) -> bool;
    
    fn consume(&self, cursor: &mut Cursor) -> Result<(K, String), String>;
}

pub struct Lexer<'a, K> {
    cursor: Cursor<'a>,
    rules: Vec<Box<dyn LexRule<K>>>,
    eof_kind: K,
}

impl<'a, K: Clone> Lexer<'a, K> {
    pub fn new(input: &'a str, eof_kind: K) -> Self {
        Self {
            cursor: Cursor::new(input),
            rules: Vec::new(),
            eof_kind,
        }
    }

    pub fn add_rule(&mut self, rule: impl LexRule<K> + 'static) {
        self.rules.push(Box::new(rule));
    }

    pub fn next_token(&mut self) -> Result<Token<K>, String> {
        self.cursor.skip_whitespace();
        let span = self.cursor.current_span();

        let next_ch = match self.cursor.peek() {
            None => {
                return Ok(Token {
                    kind: self.eof_kind.clone(),
                    span,
                    literal: String::new(),
                })
            }
            Some(&c) => c,
        };

        for rule in &self.rules {
            if rule.matches(next_ch) {
                let (kind, literal) = rule.consume(&mut self.cursor)?;
                return Ok(Token { kind, span, literal });
            }
        }

        let unknown = self.cursor.advance().unwrap();
        Err(format!(
            "Unexpected character '{}' at line {}, column {}",
            unknown, span.line, span.column
        ))
    }
}

impl<'a, K: Clone + PartialEq> Iterator for Lexer<'a, K> {
    type Item = Result<Token<K>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(token) => {
                if token.kind == self.eof_kind {
                    None
                } else {
                    Some(Ok(token))
                }
            }
            Err(err) => Some(Err(err)),
        }
    }
}

pub struct IdentifierRule<K, F>
where
    F: Fn(String) -> K,
{
    pub converter: F,
}

impl<K, F> LexRule<K> for IdentifierRule<K, F>
where
    F: Fn(String) -> K,
{
    fn matches(&self, next_char: char) -> bool {
        next_char.is_alphabetic() || next_char == '_'
    }

    fn consume(&self, cursor: &mut Cursor) -> Result<(K, String), String> {
        let mut literal = String::new();
        while let Some(&ch) = cursor.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                literal.push(cursor.advance().unwrap());
            } else {
                break;
            }
        }
        let kind = (self.converter)(literal.clone());
        Ok((kind, literal))
    }
}

pub struct IntLiteralRule<K, F>
where
    F: Fn(i64) -> K,
{
    pub converter: F,
}

impl<K, F> LexRule<K> for IntLiteralRule<K, F>
where
    F: Fn(i64) -> K,
{
    fn matches(&self, next_char: char) -> bool {
        next_char.is_ascii_digit()
    }

    fn consume(&self, cursor: &mut Cursor) -> Result<(K, String), String> {
        let mut literal = String::new();
        while let Some(&ch) = cursor.peek() {
            if ch.is_ascii_digit() {
                literal.push(cursor.advance().unwrap());
            } else {
                break;
            }
        }
        let val = literal.parse::<i64>().map_err(|e| e.to_string())?;
        Ok(((self.converter)(val), literal))
    }
}

pub struct StringLiteralRule<K, F>
where
    F: Fn(String) -> K,
{
    pub converter: F,
}

impl<K, F> LexRule<K> for StringLiteralRule<K, F>
where
    F: Fn(String) -> K,
{
    fn matches(&self, next_char: char) -> bool {
        next_char == '"'
    }

    fn consume(&self, cursor: &mut Cursor) -> Result<(K, String), String> {
        let start_span = cursor.current_span();
        cursor.advance();

        let mut content = String::new();
        let mut terminated = false;

        while let Some(ch) = cursor.advance() {
            if ch == '"' {
                terminated = true;
                break;
            }
            content.push(ch);
        }

        if !terminated {
            return Err(format!(
                "Unterminated string literal starting at line {}, column {}",
                start_span.line, start_span.column
            ));
        }

        let kind = (self.converter)(content.clone());
        Ok((kind, content))
    }
}

pub struct OperatorRule<K> {
    pub table: Vec<(String, K)>,
}

impl<K: Clone> LexRule<K> for OperatorRule<K> {
    fn matches(&self, next_char: char) -> bool {
        self.table.iter().any(|(op, _)| op.starts_with(next_char))
    }

    fn consume(&self, cursor: &mut Cursor) -> Result<(K, String), String> {
        if let Some(&first) = cursor.peek() {
            if let Some(second) = cursor.peek_second() {
                let mut two_char_op = String::new();
                two_char_op.push(first);
                two_char_op.push(second);

                if let Some((op, kind)) = self.table.iter().find(|(op, _)| *op == two_char_op) {
                    cursor.advance();
                    cursor.advance();
                    return Ok((kind.clone(), op.clone()));
                }
            }

            let one_char_op = first.to_string();
            if let Some((op, kind)) = self.table.iter().find(|(op, _)| *op == one_char_op) {
                cursor.advance();
                return Ok((kind.clone(), op.clone()));
            }
        }

        let span = cursor.current_span();
        Err(format!("Invalid operator prefix at {:?}", span))
    }
}
