rust
     let mut cs = CallStack::new();
     let mut in_frames = false;
     for line in s.lines() {
            if f(line) && !in_frames {
                // call stack starts
                in_frames = true;
                cs = CallStack::new();

                continue;
            }
            if g(line) && in_frames {
                // call stack ends
                css.push(cs);
                cs = CallStack::new();
                in_frames = false;
                continue;
            }

}

