
match value {
    0..0x7F => leb_write(),
    0x80... => leb_write(), leb_write()
}
