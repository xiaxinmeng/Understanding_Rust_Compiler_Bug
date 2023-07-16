rust
    #![feature(lint_reasons)]
    fn main() {
        #[expect(unused_variables, reason = "WIP, I'll use this value later")]
        let message = "How much wood would a woodchuck chuck?";
        
        #[expect(unused_variables, reason = "is this unused?")]
        let answer = "about 700 pounds";
        println!("A: {answer}")
    }
    