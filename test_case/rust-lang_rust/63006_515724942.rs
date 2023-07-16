rust
        let order = self.order.get() as usize;
        let mut last = buf;
        buf.windows(order).skip(1).for_each(|win| {
            self.add_ngram(last);
            last = win;
        });
        last // return trailing bytes to prepend to next chunk
