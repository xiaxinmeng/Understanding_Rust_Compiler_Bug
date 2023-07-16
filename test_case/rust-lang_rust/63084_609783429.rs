rust
// test1 is some variable
mylog::log!("Look what I got: {}", &test1);

//
// Expands to:
//

const fn get_type_str<T>(_: &T) -> &'static str {
    // type_name needs to be a const fn for this to work
    core::any::type_name::<T>()
}

// ugly hack to tranform `&'static str` -> `[u8; str.len()]`
union Transmute<T: Copy, U: Copy> {
    from: T,
    to: U,
}

const FMT: &'static str = "Look what I got: {}";
const TYP: &'static str = get_type_str(&test1);

// Transform string into byte arrays at compile time so 
// they can be placed in a specific memory region

// Formating string placed in a specific linker section
#[link_section = ".some_info_section"]
static F: [u8; FMT.as_bytes().len()] = unsafe {
    *Transmute::<*const [u8; FMT.len()], &[u8; FMT.as_bytes().len()]> {
        from: FMT.as_ptr() as *const [u8; FMT.as_bytes().len()],
    }
    .to
};

// Type string placed in a specific linker section
#[link_section = ".some_info_section"]
static T: [u8; TYP.as_bytes().len()] = unsafe {
    *Transmute::<*const [u8; TYP.len()], &[u8; TYP.as_bytes().len()]> {
        from: TYP.as_ptr() as *const [u8; TYP.as_bytes().len()],
    }
    .to
};

// Here we send where the strings are stored in the ELF + the data to the host 
write_frame_to_host(&F, &T, &test1)
