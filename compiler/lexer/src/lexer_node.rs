use popdack::lex::*;

#[lexer]
pub struct Number;
impl LexRule for Number {
    fn lparse(&mut self, input: &mut String) -> LexResult {
        let mut res = String::new();
        while let Some(c) = input.chars().next() {
            if c.is_numeric() {
                *input = skip(input, 1);
                res.push(c);
            } else {
                break;
            }
        }
        if res.is_empty() {
            LexResult::None
        } else {
            LexResult::Some(res)
        }
    }
}

#[lexer]
pub struct Ident;
impl LexRule for Ident {
    fn lparse(&mut self, input: &mut String) -> LexResult {
        let mut res = String::new();
        while let Some(c) = input.chars().next() {
            if c.is_alphabetic() {
                *input = skip(input, 1);
                res.push(c);
            } else {
                break;
            }
        }
        if res.is_empty() {
            LexResult::None
        } else {
            LexResult::Some(res)
        }
    }
}

#[lexer]
pub struct Let;
impl LexRule for Let {
    fn lparse(&mut self, input: &mut String) -> LexResult {
        let mut res = vec![];
        let mut vname = String::new();
        if input.starts_with("let") {
            *input = skip(input, 3);
            *input = skip_while(input, |c| *c == ' ');
            res.push(String::from("let"));
            while let Some(c) = input.chars().next() {
                if c.is_alphabetic() {
                    *input = skip(input, 1);
                    vname.push(c);
                } else {
                    break;
                }
            }
            *input = skip_while(input, |c| *c == ' ');
            if input.starts_with("=") {
                *input = skip(input, 1);
                *input = skip_while(input, |c| *c == ' ');
                if vname.is_empty() {
                    LexResult::None
                } else {
                    res.push(vname);
                    LexResult::Array(res)
                }
            } else {
                LexResult::None
            }
        } else {
            LexResult::None
        }
    }
}
