
fn foo<T: Str>(msg: T) {
    std::io::stdio::print(msg.as_slice());
}
foo("hello");
foo(" world".to_string());
