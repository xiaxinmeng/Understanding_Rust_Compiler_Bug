rust
impl<'a> Parse<'a> for Intensity {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            value(Intensity::Normal, tag("normal")),
            value(Intensity::Bold, tag("bold")),
            value(Intensity::Faint, tag("faint")),
        ))(input)
    }
}
