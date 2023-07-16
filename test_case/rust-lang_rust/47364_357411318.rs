rust
#[macro_use]
extern crate nom; // nom = "1.2.4" in [dependencies]
use nom::multispace;

pub enum ConditionExpression {
    Field(String),
    Placeholder,
}

pub fn condition_expr<'a>(i: &'a [u8]) -> nom::IResult<&[u8], ConditionExpression, u32> {
    nom::IResult::Done(i, ConditionExpression::Placeholder)
}

named!(pub selection<&[u8], Option<ConditionExpression>>,
    chain!(
        select: chain!(
            tag!("x") ~
            cond: opt!(complete!(chain!(
                multispace? ~
                cond: condition_expr,
                || { cond }
            ))) ~
            || { cond }
        ) ~
        tag!(";"),
        || { select }
    )
);

fn main() {
    selection("x ".as_bytes());
}
