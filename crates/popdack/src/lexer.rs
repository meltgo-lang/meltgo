/*
 * Copyright (c) 2026 Cargry Language
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub enum LexResult {
    Some(String),
    Array(Vec<String>),
    None,
}

pub trait LexRule: LexClone + LexIgnore {
    fn lparse(&mut self, input: &mut String) -> LexResult;
}

pub trait LexerManager {
    fn new() -> Self;
    fn run(&mut self, input: &str);
}

pub trait LexClone {
    fn clone_box(&self) -> Box<dyn LexRule>;
}

pub trait LexIgnore {
    fn is_ignore(&self) -> bool;
}

impl Clone for Box<dyn LexRule> {
    fn clone(&self) -> Box<dyn LexRule> {
        self.clone_box()
    }
}

pub struct Lexer {
    rules: Vec<Box<dyn LexRule>>,
    tokens: Vec<String>,
    rule_pos: Vec<(Box<dyn LexRule>, usize)>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            rules: vec![],
            tokens: vec![],
            rule_pos: vec![],
        }
    }

    pub fn add_rule(&mut self, rule: impl LexRule + Clone + LexClone + LexIgnore + 'static) {
        self.rules.push(Box::new(rule));
    }

    fn excute(&mut self, input: &mut String) -> (LexResult, bool) {
        let index = self.tokens.len();
        for rule in self.rules.iter_mut() {
            let lparsed = rule.lparse(input);
            match lparsed {
                LexResult::Some(_) => {
                    if rule.is_ignore() {
                        return (LexResult::None, true);
                    } else {
                        self.rule_pos.push((rule.clone(), index));
                    }
                    return (lparsed, rule.is_ignore());
                }
                LexResult::Array(_) => {
                    if rule.is_ignore() {
                        return (LexResult::None, true);
                    } else {
                        self.rule_pos.push((rule.clone(), index));
                    }
                    return (lparsed, rule.is_ignore());
                }
                LexResult::None => (),
            }
        }
        (LexResult::None, false)
    }

    pub fn run(&mut self, input: &str) {
        let mut input = input.to_string();
        while !input.is_empty() {
            let (res, is_ignore) = self.excute(&mut input);
            match res {
                LexResult::Some(w) => self.tokens.push(w),
                LexResult::Array(vec) => {
                    for item in vec.iter() {
                        self.tokens.push(item.clone());
                    }
                }
                LexResult::None => {
                    if !is_ignore {
                        break;
                    }
                }
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
