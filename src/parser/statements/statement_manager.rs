use std::sync::OnceLock;

use regex::Regex;

use crate::parser::statements::statement::Statement;

pub struct StatementManager {
    source_code: Vec<String>,
    statements: Vec<Statement>,
}

impl StatementManager {
    pub fn new(source_code: &str) -> Self {
        static RE: OnceLock<Regex> = OnceLock::new();
        let regex = RE.get_or_init(|| Regex::new("[^;{}]").unwrap());
        Self {
            source_code: regex
                .split(source_code)
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            statements: vec![],
        }
    }
    pub fn marking(&mut self) {
        for s in self.source_code.iter() {
            self.statements.push(Statement::mapping(s.trim()));
        }
    }
}
