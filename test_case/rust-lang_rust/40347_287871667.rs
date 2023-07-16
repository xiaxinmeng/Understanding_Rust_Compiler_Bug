rust
               args.iter()
                   .map(|&v| format!("{:?}", Value(v)))
                   .collect::<Vec<String>>()
                   .join(", ")
