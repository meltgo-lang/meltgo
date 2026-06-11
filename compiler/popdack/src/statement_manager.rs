use std::sync::OnceLock;

use regex::Regex;

use super::statement::Statement;

#[derive(Debug)]
pub struct StatementManager<'a> {
    source_code: &'a str,
    cutted_code: Vec<String>,
    statements: Vec<Statement>,
}

impl<'a> StatementManager<'a> {
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
            self.statements.push(Statement::mapping(s));
        }
    }

    pub fn get_statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}
