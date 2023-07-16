
pub fn needs_many_args() {
    unsafe {
        core::arch::asm!(
            "mov {a0}, {a0}",
            "mov {a1}, {a1}",
            "mov {a2}, {a2}",
            "mov {a3}, {a3}",
            "mov {a4}, {a4}",
            "mov {a5}, {a5}",
            "mov {a6}, {a6}",
            "mov r8, r8",
            a0 = in(reg) 0,
            a1 = in(reg) 1,
            a2 = in(reg) 2,
            a3 = in(reg) 3,
            a4 = in(reg) 4,
            a5 = in(reg) 5,
            a6 = in(reg) 6,
            in("r8") 7,
        );
    }
}
