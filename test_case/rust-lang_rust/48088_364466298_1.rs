
#[no_mangle]
#[unwind]
pub extern "fastcall" fn eval(ast: &AST, interpreter: &Interpreter) -> AST {
...
}
