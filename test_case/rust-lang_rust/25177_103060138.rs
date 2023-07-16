 rust
        let thing = match (1, 2) {
            (_, _) => foo,
        }
        (3, 4);
//       ^
//       | : Parser is here, it has just read the `(`.
