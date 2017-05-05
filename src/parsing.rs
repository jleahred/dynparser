
#[derive(Default, Debug, PartialEq)]
pub struct Parsing {
    pub position: ParsingPossition,
    pub parsing_text: ParsingText,
}




#[derive(Debug, PartialEq, Default)]
pub struct ParsingPossition {
    pub n: usize,
    pub col: usize,
    pub row: usize,
}


#[derive(Debug, PartialEq, Default)]
pub struct ParsingText(String);
impl ParsingText {
    pub fn new(txt: &str) -> Self {
        ParsingText(txt.to_owned())
    }
    pub fn string(&self) -> String {
        self.0.clone()
    }
}

pub fn parsing_text(txt: &str) -> ParsingText {
    ParsingText(txt.to_owned())
}



impl Parsing {
    pub fn new(s: &str) -> Self {
        Parsing { parsing_text: ParsingText(s.to_owned()), ..Parsing::default() }
    }
}
