rs
#![feature(try_blocks)]
#![feature(let_else)]
#![feature(never_type)]

fn main() {
    let _: Result<i32, i32> = try {
        let Some(x) = Some(0) else {
            Err::<!, _>(1)?
        };

        x
    };
}
