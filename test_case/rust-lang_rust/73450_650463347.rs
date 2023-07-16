rust
        asm!("
            mov {save11}, r11
            mov r11, {in11}
            svc #0
            mov r11, {save11}
            ",
            save11 = out(reg) _,
            in11 = in(reg) something_or_other,

            options(preserves_flags)
        );
