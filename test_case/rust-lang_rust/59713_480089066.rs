rust
fn get_abc(input: &[u8]) -> IResult<&[u8], &[u8], u32> {
    let (i, o) = try_parse!(input,
      add_return_error!(
        ErrorKind::Custom(1000), tag!("abc")
      )
    );
    Ok((i, o))
}

// ---------


// Here, the `error` type may be confusing because the compiler accepts the following syntax:
// `Err<P>`.
fn give_error_message<P: Clone, E: Clone>(error: Err<P> /*or Err<P, E=u32>*/) -> &'static str {
    match &error {
        nom::Err::Error(ref c) => {
            let vector: Vec<(P, ErrorKind<E>)> = error_to_list(c);
        },
        _ => unreachable!()
    };

    ""
}

// ---------

fn main() {
    let bar = "def";
    let _ = get_abc(bar.as_bytes())
        .map_err(|e| {
            let error_message = give_error_message(e);
        });
}
