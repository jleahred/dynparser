use atom::Atom;
use parser::Parse;
use parser;
use Text2Parse;


#[derive(Debug, PartialEq)]
pub enum Expression {
    Simple(Atom),
    Or(MultiExpr),
    And(MultiExpr),
}



#[derive(Debug, PartialEq)]
pub struct MultiExpr(pub Vec<Expression>);





impl Parse for Expression {
    fn parse(&self,
             text2parse: &Text2Parse,
             pars_pos: parser::Possition)
             -> Result<parser::Possition, String> {
        match self {
            &Expression::Simple(ref atom) => atom.parse(text2parse, pars_pos),
            &Expression::Or(MultiExpr(ref exprs)) => parse_or(text2parse, exprs, pars_pos),
            _ => Err("pending implementation".to_owned()),
        }
    }
}


fn parse_or(text2parse: &Text2Parse,
            exprs: &Vec<Expression>,
            pars_pos: parser::Possition)
            -> Result<parser::Possition, String> {

    for e in exprs {
        match e.parse(text2parse, pars_pos.clone()) {
            Ok(p) => return Ok(p),
            _ => (),
        }
    }

    Err("failed on or, pending getting best option for error message".to_owned())
}
