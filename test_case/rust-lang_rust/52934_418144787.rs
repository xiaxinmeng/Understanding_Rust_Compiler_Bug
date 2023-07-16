rust
#![allow(dead_code)]

#[derive(Copy, Clone)]
struct BodyId;
struct AnonConst { body: BodyId }
enum ExprKind { Closure(BodyId), Repeat(AnonConst), Others }
struct Expr { node: ExprKind, }

struct Body { value: Expr }

struct Map;
impl Map { fn body(&self, _: BodyId) -> &Body { loop { } } }

struct SpanlessHash<'a> { cx: &'a Map, }

impl <'a> SpanlessHash<'a> {
    fn hash_expr(&mut self, e: &Expr) {
        match e.node {
            ExprKind::Closure(eid) => {
                self.hash_expr(&self.cx.body(eid).value);
            },
            ExprKind::Repeat(ref l_id) => {
                self.hash_expr(&self.cx.body(l_id.body).value);
            },
            _ => {}
        }
    }
}

struct Config;
trait LateLintPass<'a> { }
type LateLintPassObject = Box<dyn for<'a> LateLintPass<'a>>;

struct TriviallyCopyPassByRef { }
impl TriviallyCopyPassByRef { fn new(_: &Config) -> Self { loop { } } }
impl<'a> LateLintPass<'a> for TriviallyCopyPassByRef { }

struct Session { target: Config }
struct Registry<'a> { sess: &'a Session }
impl<'a> Registry<'a> {
    fn register_late_lint_pass(&mut self, _: LateLintPassObject) { loop { } }
}

fn register_plugins(reg: &mut Registry<'_>) {
    reg.register_late_lint_pass(Box::new(TriviallyCopyPassByRef::new(&reg.sess.target)));
}

fn main() { }
