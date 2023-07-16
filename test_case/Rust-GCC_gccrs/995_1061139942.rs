rust
enum NewTypeOrStruct {
    One(i32),
    Two{other: String, value: i32},
} 

fn foo((NewTypeOrStruct::One(foo) | NewTypeOrStruct::Two{other: _, value: foo}): NewTypeOrStruct) -> i32 {
    foo + 15i32
}

fn main() {
    foo(NewTypeOrStruct::One(15));
    foo(NewTypeOrStruct::Two {
        other: String::from("other"),
        value: 14
    });
}
