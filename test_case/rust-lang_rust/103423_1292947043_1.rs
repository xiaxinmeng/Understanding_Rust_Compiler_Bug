
        block_on(async {
            let iter1 = create_iterator_6(1);
            let iter1 = next(iter1);
            pin_mut!(iter1);
            while let Some(_) = iter1.next().await {}
        });
