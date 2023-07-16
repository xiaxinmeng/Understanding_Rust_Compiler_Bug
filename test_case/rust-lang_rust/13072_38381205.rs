 rust
#[repr(u32)]
enum InitFlags {
    InitTimer               = 0x00000001,
    InitAudio               = 0x00000010,
    InitVideo               = 0x00000020,
    InitJoystick            = 0x00000200,
    InitHaptic              = 0x00001000,
    InitGamecontroller      = 0x00002000,
    InitEvents              = 0x00004000,
    InitNoparachute         = 0x00100000,
    InitEverything          = InitTimer as u32
                            | InitAudio as u32
                            | InitVideo as u32
                            | InitEvents as u32
                            | InitJoystick as u32
                            | InitHaptic as u32
                            | InitGamecontroller as u32,
}

impl CLike for InitFlags {
    fn to_uint(&self) -> uint {
        *self as uint
    }

    fn from_uint(v: uint) -> InitFlags {
        unsafe { cast::transmute(v as u32) }
    }
}

// ...

let mut flags = EnumSet::empty();
flags.add(sdl2::InitVideo);
flags.add(sdl2::InitEvents);
sdl2::init(flags)
