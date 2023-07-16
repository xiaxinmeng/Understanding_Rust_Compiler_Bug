rust
macro_rules! exp {
    (const $n:expr) => {
        Box::new(Exp::Const($n))
    };
}

macro_rules! stmt {
    (exp $e:expr) => {
        Box::new(Stmt::Exp($e))
    };
    (exp $($t:tt)+) => {
        Box::new(Stmt::Exp(exp!($($t)+)))
    };
}

macro_rules! seq {
    ($stmt1:expr; $stmt2:expr $(;)?) => {
        Box::new(Stmt::Seq($stmt1, $stmt2))
    };
    ($stmt1:expr; $stmt2:expr; $($stmt:expr);+ $(;)?) => {
        seq!(seq!($stmt1; $stmt2); $($stmt);+)
    };
}

fn main() {
    let seq = seq!(
        stmt!(exp const 1);
        stmt!(exp const 2);
        stmt!(exp const 3);
    );
}
