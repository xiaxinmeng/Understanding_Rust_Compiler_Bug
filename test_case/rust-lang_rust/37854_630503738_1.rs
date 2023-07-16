rust
let tshirt_width = 20;
let tshirt_size = match tshirt_width {
    16 => "S",       // 16
    17 | 18 => "M",  // 17 or 18
    19..21 => "L",   // 19 to 21; 21 excusive (19,20)
    21..=24 => "xL", // 21 to 24; 24 inclusive (21,22,23,24)
    25 => "XXL",
    _ => "Not Available",
};
