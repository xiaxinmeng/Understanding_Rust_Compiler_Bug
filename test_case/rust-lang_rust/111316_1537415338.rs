rs
match foo {
            tmp => {
                eprintln!("[{}:{}] {} = {:#?}", file!(), line!(), stringify!(foo), &tmp);
                tmp
            }
        }
