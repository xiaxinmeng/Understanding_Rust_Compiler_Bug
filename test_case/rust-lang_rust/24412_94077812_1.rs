
    let byte = bit >> 5; // "byte" seems like a misnomer here
    let lobits = 1 << (bit & 0xb11111); // or 0x1F if you prefer that
