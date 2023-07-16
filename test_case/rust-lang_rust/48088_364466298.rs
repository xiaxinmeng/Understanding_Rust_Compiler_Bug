
#[allow(improper_ctypes)]
extern "fastcall" {
    #[unwind]
    pub fn eval(ast: &AST, interpreter: &Interpreter) -> AST;
 ...
