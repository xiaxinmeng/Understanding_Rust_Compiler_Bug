rust
type Test<'a, 'b: 'static> = &'a mut &'b mut i32;

fn main() {
    let x : Test = &mut &mut 0;
}
