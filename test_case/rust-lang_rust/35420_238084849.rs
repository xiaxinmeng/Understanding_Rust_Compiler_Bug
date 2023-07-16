
    unsafe {
        asm!("mov $1, $0" : "=r"(x) : "r"(5));
        //~^ ERROR re-assignment of immutable variable `x`
        //~| NOTE re-assignment of immutable
        //~| NOTE in this expansion of asm!
    }
