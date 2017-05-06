// #![allow(dead_code)]
// pending... remove


// extern crate indentation_flattener;
// use indentation_flattener::flatter

use {Symbol, Rules, Text2Parse, Error, error};



pub trait Parse {
    fn parse(&self, text2parse: &Text2Parse, pars_pos: Possition) -> Result<Possition, Error>;
}



pub fn parse(text2parse: &Text2Parse,
             symbol: &Symbol,
             pars_pos: Possition,
             rules: &Rules)
             -> Result<Possition, Error> {
    let pars_pos = rules.get(symbol)
        .ok_or(error(&pars_pos, "undefined symbol"))?
        .parse(text2parse, pars_pos)?;

    if pars_pos.n == text2parse.string().len() {
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
