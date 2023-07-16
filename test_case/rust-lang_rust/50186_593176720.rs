rust
        let mut cu1 = b.copy_update();
        let mut cu2 = b.copy_update();
        *cu1 += 1;
        *cu2 += 1;
        *cu1 += *cu2;
