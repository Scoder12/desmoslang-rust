#[derive(Clone, Debug, PartialEq)]
pub enum AST<'a> {
    Call(&'a str, Vec<&'a str>),
    Num(&'a str), // We don't care about the value of the int, desmos can figure that out
    Add(Box<AST<'a>>, Box<AST<'a>>),
}
