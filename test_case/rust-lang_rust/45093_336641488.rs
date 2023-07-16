rust
let result = sub_generator.resume();
match result {
    GeneratorState::Yielded(x) => {
        yield x;
    }
    _ => panic!("unexpected value from resume"),
}
