// #![allow(dead_code)]
// pending... remove


// extern crate indentation_flattener;
// use indentation_flattener::flatter

use Symbol;
use Rules;
use Text2Parse;



pub trait Parse {
    fn parse(&self, text2parse: &Text2Parse, pars_pos: Possition) -> Result<Possition, String>;
}



pub fn parse(text2parse: &Text2Parse,
             symbol: &Symbol,
             pars_pos: Possition,
             rules: &Rules)
             -> Result<Possition, String> {
    let pars_pos = rules.get(symbol)
        .ok_or("undefined symbol")?
        .parse(text2parse, pars_pos)?;

    if pars_pos.n == text2parse.string().len() {
        Ok(pars_pos)
    } else {
        Err(format!("not consumed full input"))
    }
}


#[derive(Debug, PartialEq, Default, Clone)]
pub struct Possition {
    pub n: usize,
    pub col: usize,
    pub row: usize,
}




impl Possition {
    pub fn new() -> Self {
        Possition { ..Possition::default() }
    }
}
