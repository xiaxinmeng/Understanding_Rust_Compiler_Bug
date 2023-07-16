
fn main() {
    tag foo { bar(str, {node: int, span: int}); baz; }
    alt baz {
      bar("foo", {node: _, _}) {}
    }
}
