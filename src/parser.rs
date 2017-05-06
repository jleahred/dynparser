// #![allow(dead_code)]
// pending... remove


// extern crate indentation_flattener;
// use indentation_flattener::flatter

use {Symbol, Rules, Text2Parse, Error, error};


pub struct Config<'a> {
    pub text2parse: &'a Text2Parse,
    pub rules: &'a Rules,
}




pub trait Parse {
    // fn parse(&self, text2parse: &Text2Parse, pars_pos: Possition) -> Result<Possition, Error>;
    fn parse(&self, pars_conf: &Config, pars_pos: Possition) -> Result<Possition, Error>;
}



pub fn parse(pars_conf: &Config, symbol: &Symbol, pars_pos: Possition) -> Result<Possition, Error> {
    let pars_pos = pars_conf.rules
        .get(symbol)
        .ok_or(error(&pars_pos, "undefined symbol"))?
        .parse(pars_conf, pars_pos)?;

    if pars_pos.n == pars_conf.text2parse.string().len() {
        Ok(pars_pos)
    } else {
        Err(error(&pars_pos, "not consumed full input"))
    }
}


#[derive(Debug, PartialEq, Default, Clone, Eq, PartialOrd, Ord)]
pub struct Possition {
    pub n: usize,
    pub col: usize,
    pub row: usize,
}


#[derive(Debug, PartialEq, Default, Clone)]
pub struct Status {
    pub pos: Possition,
    pub deep_error: Error,
}




impl Possition {
    pub fn new() -> Self {
        Possition { ..Possition::default() }
    }
}
