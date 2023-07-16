
pub fn midi_codec() -> impl Codec<Value = Midi> + Clone {
    MidiCodec
}
