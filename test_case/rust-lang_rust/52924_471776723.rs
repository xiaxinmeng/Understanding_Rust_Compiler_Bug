rust
    let composed_2 = async {
        let inner = i_am_1kb();
        println!("");
        await!(inner);
    };
