rust
            .partition_in_place(|&element| matches!(cmp(element, pivot), Ordering::Less))
