rust
#[inert]
mac! {
    struct S;
}

=>

mac! {
    #![inert]

    struct S;
}
