rust
struct Command<'s> {
    session: &'s (),
    imp: std::convert::Infallible,
}

fn command(_: &()) -> Command<'_> {
    unreachable!()
}

fn with_session<'s>(a: &std::process::Command, b: &'s ()) -> Command<'s> {
    a.get_program();
    command(b)
}
