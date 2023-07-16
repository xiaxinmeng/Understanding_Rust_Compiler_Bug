rs
        byte_list.iter_mut().find_map(|item| {
            Bigger::<'a>::other(item); // <- expansion of the `Self::other`
            Some(())
        });
