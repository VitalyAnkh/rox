use std::collections::HashMap;
use crate::token::Token;

#[derive(Debug,PartialEq, Eq)]
pub struct Scanner{
  source: String,
  tokens: Vec<Token>,
  start: u64,
  current: u64,
  line: u64
}

impl Scanner {
   pub fn new(contents: String) -> Self{
    Self { source: contents, tokens: vec![], start: 0, current: 0, line: 0 }
   }
   pub fn scan_tokens(&mut self)->Vec<Token>{
    todo!()
   }
}

