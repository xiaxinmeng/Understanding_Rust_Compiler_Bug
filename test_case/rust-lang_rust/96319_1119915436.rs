rust
pub struct Stmt {
    pub stmt_type: StmtKind,
    pub stmt_tag: Option<LintTag>,
}
pub struct LintTag;
pub enum StmtKind {
    If(If),
    Block(&'static str),
    Return(Return),
}
pub struct If {
    pub condition: Function,
}
pub struct Return {
    pub value: Function,
}
pub struct Function {
    pub parameters: Box<Stmt>,
}
pub fn start_late_pass(stmt_receiver: Box<Stmt>) {
    spawn(async { stmt_receiver });
}

pub fn spawn<T>(_: T)
where
    T: Send,
{
}

