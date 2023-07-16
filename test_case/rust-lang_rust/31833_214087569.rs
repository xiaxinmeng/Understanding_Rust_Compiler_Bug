
#[derive(Clone)]
enum Type {
    Function(Type),
    Term(Type)
}
