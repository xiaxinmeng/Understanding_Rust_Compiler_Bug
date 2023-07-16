rust
    #![feature(lint_reasons)]
    fn main() {
        #[deny(unused_variables, reason = "unused variables, should be removed")]
        let unused = "How much wood would a woodchuck chuck?";
    }
    