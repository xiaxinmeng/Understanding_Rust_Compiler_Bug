 rust
#[crate_id="abc"];
#[crate_type="bin"];
#[feature(macro_rules)];

#[macro_export]
macro_rules! path(
    ($e:expr) => (&Path::new($e));
    ($($arg:tt),+) => (
        &Path::new(format_args!(::std::fmt::format, $($arg),+ ))
    )
)

fn main(){
    use std::io::{File,Open,ReadWrite};

    for num in range(0,10){
        let p = path!("message{}.txt",num);
        println!("write file {}", "message" + num.to_str() + ".txt" );
        let mut file = match File::open_mode( p, Open,ReadWrite ){
            Ok(o) => o,
            Err(e) => fail!("can't open file")
        };
        file.write_line( "one line" );
    }
}


