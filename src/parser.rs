use {Symbol, Rules, Text2Parse, Error, error};
use parser;


pub struct Config<'a> {
    pub text2parse: &'a Text2Parse,
    pub rules: &'a Rules,
}




pub trait Parse {
    fn parse(&self, conf: &Config, status: parser::Status) -> Result<parser::Status, Error>;
}



pub fn parse(conf: &Config, symbol: &Symbol, status: parser::Status) -> Result<Status, Error> {
    let status = conf.rules
        .get(symbol)
        .ok_or(error(&status.pos, &format!("undefined symbol {:?}", symbol)))?
        .parse(conf, status)?;

    if status.pos.n == conf.text2parse.string().len() {
        Ok(status)
    } else {
        Err(error(&status.pos, "not consumed full input"))
    }
}


#[derive(Debug, PartialEq, Default, Clone, Eq, PartialOrd, Ord)]
pub struct Possition {
    pub n: usize,
    pub col: usize,
    pub row: usize,
}

#[derive(Debug, PartialEq, Default, Clone, PartialOrd)]
pub struct Depth(pub u32);

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Status {
    pub pos: Possition,
    pub depth: Depth,
    pub deep_error: Option<Error>,
}


impl Status {
    pub fn new() -> Self {
        Status {
            pos: Possition::new(),
            depth: Depth(0),
            deep_error: None,
        }
    }
}



impl Possition {
    pub fn new() -> Self {
        Possition { ..Possition::default() }
    }
}
