
loop {
    // whatever
    loop {
        // whatever
        loop {
            if whatever {
                break 0;  // breaks to the parent loop
            }
            else {
                break 1;  // breaks to the grandparent loop
            }
        }
    }
}
