 diff
        thread::spawn(move || {
+           println!("{} starting", p.name);
            p.eat(&table);
        })
