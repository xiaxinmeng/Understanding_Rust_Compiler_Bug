rust

// This is broken
#[derive(AstNode, Fold, Clone, Debug, PartialEq)]
pub struct Class {
    pub span: Span,

    pub body: Vec<ClassMethod>,
    pub super_class: Option<Box<Expr>>,
}

// This works
#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    pub span: Span,

    pub body: Vec<ClassMethod>,
    pub super_class: Option<Box<Expr>>,
}

