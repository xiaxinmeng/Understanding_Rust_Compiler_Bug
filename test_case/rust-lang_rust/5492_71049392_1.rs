 rust
#[repr(C)]
pub struct MOUSEINPUT { ... }
#[repr(C)]
pub struct KEYBDINPUT { ... }
#[repr(C)]
pub struct HARDWAREINPUT { ... }

#[repr(C)]
pub struct INPUT {
    pub tag_: DWORD,
    pub union_: MOUSEINPUT, // MOUSEINPUT largest and most aligned
}
