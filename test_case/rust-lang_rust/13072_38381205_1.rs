 rust
bitflags!(InitFlags: Uint32 {
    InitTimer               = 0x00000001,
    InitAudio               = 0x00000010,
    InitVideo               = 0x00000020,
    InitJoystick            = 0x00000200,
    InitHaptic              = 0x00001000,
    InitGamecontroller      = 0x00002000,
    InitEvents              = 0x00004000,
    InitNoparachute         = 0x00100000,
    InitEverything          = InitTimer.bits
                            | InitAudio.bits
                            | InitVideo.bits
                            | InitEvents.bits
                            | InitJoystick.bits
                            | InitHaptic.bits
                            | InitGamecontroller.bits
})

// ...

sdl2::init(sdl2::InitVideo | sdl2::InitEvents)
