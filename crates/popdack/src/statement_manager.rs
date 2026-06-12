use std::sync::OnceLock;

use regex::Regex;

pub trait Statement<T> {
    fn mapping(input: &str) -> T;
}

#[derive(Debug)]
pub struct StatementManager<'a, T> {
    source_code: &'a str,
    cutted_code: Vec<String>,
    statements: Vec<T>,
}

impl<'a, T: Statement<T>> StatementManager<'a, T> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            source_code,
            cutted_code: source_code
                .replace("{", "{;")
                .replace("}", "};")
                .replace("//", ";//")
                .replace("/*", ";/*")
                .replace("*/", "/*;")
                .split(';')
                .filter(|s| *s != "")
                .map(|x| x.trim().to_string())
                .collect::<Vec<String>>(),
            statements: vec![],
        }
    }

    pub fn marking(&mut self) {
        for s in self.cutted_code.iter() {
            self.statements.push(T::mapping(s));
        }
    }

    pub fn get_statements(&self) -> &Vec<T> {
        &self.statements
    }
}
