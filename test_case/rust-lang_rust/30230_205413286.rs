 rust
fn main() {
    AppendableValue::Declaration(&PropertyDeclaration)
}

enum AppendableValue<'a, I = ::std::iter::Empty< &'a PropertyDeclaration>>
where I: Iterator<Item=&'a PropertyDeclaration> {
    Declaration(&'a PropertyDeclaration),
    DeclarationsForShorthand(I),
}

struct PropertyDeclaration;
