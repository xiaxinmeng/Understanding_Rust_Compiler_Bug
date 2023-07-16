rust
#![feature(decl_macro)]

struct FromMacro;
pub macro fun_with_macros() {
    fn my_function() -> FromMacro { FromMacro }

    fn uses_from_macro() {
        let FromMacro = Self::my_function();
    }
}

struct LocalStruct;
struct NoMacro;
impl LocalStruct {
    fun_with_macros!();

    fn my_function() -> NoMacro { NoMacro }

    fn doesnt_use_macro() {
        let NoMacro = Self::my_function();
    }
}
