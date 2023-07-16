rust
mod logic {
    pub struct Expr(std::rc::Rc<ExprContent>);
    
    enum ExprContent {
        A(Vec<Expr>),
        B(Vec<Expr>),
        C(Vec<Expr>),
        D(Vec<Expr>),
        E(Vec<Expr>),
        F(Vec<Expr>),
        G(Vec<Expr>),
    }
    
    pub trait ExprVisitor {
        fn a(&self, e: &[Expr]);
        fn b(&self, e: &[Expr]);
        fn c(&self, e: &[Expr]);
        fn d(&self, e: &[Expr]);
        fn e(&self, e: &[Expr]);
        fn f(&self, e: &[Expr]);
        fn g(&self, e: &[Expr]);
    }
    
    impl Expr {
        pub fn new() -> Expr {
            unimplemented!()
        }
        
        pub fn accept_visitor(&self, visitor: &dyn ExprVisitor) {
            match & *self.0 {
                ExprContent::A(e) => visitor.a(&e),
                ExprContent::B(e) => visitor.b(&e),
                ExprContent::C(e) => visitor.c(&e),
                ExprContent::D(e) => visitor.d(&e),
                ExprContent::E(e) => visitor.e(&e),
                ExprContent::F(e) => visitor.f(&e),
                ExprContent::G(e) => visitor.g(&e),
            }
        }
    }

    mod for_each {
        use super::{Expr, ExprVisitor};

        struct ForEach;
        
        impl Expr {
            pub fn for_each(&self) {
                self.accept_visitor(&ForEach)
            }
        }
        
        impl ExprVisitor for ForEach {
            fn a(&self, e: &[Expr]) {
                e.iter().for_each(|x| x.accept_visitor(self))
            }
            
            fn b(&self, e: &[Expr]) {
                e.iter().for_each(|x| x.accept_visitor(self))
            }
        
            fn c(&self, e: &[Expr]) {
                e.iter().for_each(|x| x.accept_visitor(self))
            }
        
            fn d(&self, e: &[Expr]) {
                e.iter().for_each(|x| x.accept_visitor(self))
            }

            fn e(&self, e: &[Expr]) {
                e.iter().for_each(|x| x.accept_visitor(self))
            }

            fn f(&self, e: &[Expr]) {
                e.iter().for_each(|x| x.accept_visitor(self))
            }

            fn g(&self, e: &[Expr]) {
                e.iter().for_each(|x| x.accept_visitor(self))
            }
        }
    }
}

fn main() {
    let expr = logic::Expr::new();
    expr.for_each();
}
