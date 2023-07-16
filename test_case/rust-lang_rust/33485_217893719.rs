
macro_rules! m {
    ($arg1: ident, $arg2: ident) => {
        {
            let mut $arg1 = $arg1;
            $arg1 += 1;
            println!("{:?}", $arg1);
            {
                // $arg1 and $arg2 share the context, so $arg2 here picks up
                // the incremented value of $arg1 and not the original argument
                let mut $arg2 = $arg2;
                $arg2 += 1;
                println!("{:?}", $arg2);
            }
        }
    }
}

fn main() {
    let mut a = 0;
    println!("{:?}", a);
    m!(a, a);
}

-------
Prints:

0
1
2
