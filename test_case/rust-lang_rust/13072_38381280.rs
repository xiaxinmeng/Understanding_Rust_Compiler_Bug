 rust
let mut flags = collections::EnumSet::empty();
flags.add(InitJoystick);
flags.add(InitVideo);
flags.add(InitHaptic);
for flag in flags.iter() {
    println!("{}", flag);
}
