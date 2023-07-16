
error[E0596]: cannot borrow immutable local variable `foo` as mutable
  --> src/main.rs:5:19
   |
4  |                let $s = 0;
   |   _________________-
5  |  |             *&mut $s = 0;
   |  |___________________^
6  | ||         }
7  | ||     }
8  | || }
9  | ||
10 | || fn main() {
11 | ||     bad!(foo whatever);
   | ||     -------^-----------
   | ||_____|______|
   | |______|______consider changing this to `mut $s = 0;
            *&mut $s = 0;
        }
    }
}

fn main() {
    bad!(foo`
   |        |      cannot borrow mutably
   |        in this macro invocation
