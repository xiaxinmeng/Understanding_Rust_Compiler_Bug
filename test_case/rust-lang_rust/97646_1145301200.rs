c
    register char al asm("al") = 0;
    register char ah asm("ah") = 1;
    asm volatile("": : "r"(ah), "r"(al));
