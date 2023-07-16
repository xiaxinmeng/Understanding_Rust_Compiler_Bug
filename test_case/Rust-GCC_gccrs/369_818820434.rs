c++
// Suppose we're trying to create a visitor for AST nodes
class ASTVisitor {
public:
    virtual void visit(Token& token) = 0;
    virtual void visit(LiteralExpr& expr) = 0;
    // etc.
};

// It's common to define operations only for a subset of types.
// A "base" visitor type would provide default "no-op" operations
class ASTVisitorBase : public ASTVisitor {
    virtual void visit(Token& token) {}
    virtual void visit(LiteralExpr& expr) {}
};

int main() {
    // Let's define an operation
    auto vis = Rust::begin_visitor<ASTVisitorBase>()
        .on<Token>([](Token& token) {
            // Token-specific logic
        })
        .on<LiteralExpr>([](LiteralExpr& expr) {
            // LiteralExpr-specific logic
        })
        .end_visitor();

    // We can now use that visitor as usual
    AST::Expr* expr = new LiteralExpr();
    expr->accept_vis(vis); // triggers LiteralExpr-specific logic

    // Visitors can't have return values(it's not specific to inline visitors)
    // You can use capture-by-reference in lambdas as a workaround.
}

// Inline visitor is merely syntax sugar--here's the "traditional" way to define such an operation.
// Notice we have to create an entire class for every operation.
class MyOperationVisitor : public ASTVisitorBase {
public:
    // Since we can't capture local variables, we have to pass in parameters explictly
    MyOperationVisitor(...) {}

    virtual void visit(Token& token) override {
        // Token-specific logic
    }

    virtual void visit(LiteralExpr& expr) override {
        // LiteralExpr-specific logic
    }

private:
    // Return values are often stored as class members and later retrieved manually.
    int return_value;
};
