// #![allow(dead_code)]
// pending... remove


// extern crate indentation_flattener;
// use indentation_flattener::flatter

use parsing::Parsing;
use expression::Expression;
use Symbol;
use Rules;





pub fn parse(symbol: &Symbol, parsing: Parsing, rules: &Rules) -> Result<Parsing, String> {
    let expr = rules.get(symbol).ok_or("undefined symbol")?;


    let parsing = match expr {
            &Expression::Atom(ref term) => term.parse(parsing),
            _ => Err("Pending implementation".to_owned()),
        }
        ?;

    if parsing.position.n == parsing.parsing_text.string().len() {
        Ok(parsing)
    } else {
        Err(format!("not consumed full input"))
    }
}
