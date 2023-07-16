rs
        byte_list.iter_mut().find_map(|item| {
            Bigger::<'_>::other(item);
            Some(())
        });
