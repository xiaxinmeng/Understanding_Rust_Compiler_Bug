python
print("""#![allow(warnings)]
enum A {""")
for i in range(8192):
    print(f"    Var{i},")
print("""}
fn main() {
    match A::Var0 {""")
for i in range(8192):
    print(f"        A::Var{i} => {{}}")
print("""    }
}""")
