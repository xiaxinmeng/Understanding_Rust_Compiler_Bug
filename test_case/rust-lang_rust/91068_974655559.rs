rs
struct S<'a: 'static>(*mut &'a ());

fn f<'a>(s: &'a str, _: <S<'a> as Trait>::Type) -> &'static str {
    s
}

// same main function…
