use atom::Atom;

#[derive(Debug, PartialEq)]
pub enum Expression {
    SubExpr(OrExpr),
    Atom(Atom),
}




#[derive(Debug, PartialEq)]
pub struct OrExpr {
    seq_expr: Vec<SeqExpr>,
}




#[derive(Debug, PartialEq)]
pub struct SeqExpr {
    par_expr: Vec<OrExpr>,
}
