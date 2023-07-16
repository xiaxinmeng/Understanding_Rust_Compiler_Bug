rust
pub trait ToOther {
	type Other<P: AsPitch, V: AsVal, B: AsPitchBend>;

	fn to_other<Pitch: AsPitch, Val: AsVal, Pb: AsPitchBend>(&self) -> Self::Other/*<Pitch, Val, Pb>*/;
}

impl<Pitch: AsPitch, Val: AsVal, Pb: AsPitchBend> ToOther for HighLvlMsg<Pitch, Val, Pb> {
	type Other<P, V, B> = HighLvlMsg<P, V, B>;

	fn to_other<Pitch2: AsPitch, Val2: AsVal, Pb2: AsPitchBend>(&self) -> Self::Other/*<Pitch2, Val2, Pb2>*/ {
		use HighLvlMsg::*;

		match *self {
			NoteOn { pitch, vel, len } => NoteOn { pitch: AsPitch::conv(pitch), vel, len },
			NoteOff { pitch, vel } => NoteOff { pitch: AsPitch::conv(pitch), vel },
			ControlChange { cc, val } => ControlChange { cc, val: AsVal::conv(val) },
			ProgramChange { prog } => ProgramChange { prog },
			PitchBend { semitones } => PitchBend { semitones: AsPitchBend::conv(semitones) },
			PolyphonicAftertouch { pitch, val } => PolyphonicAftertouch { pitch: AsPitch::conv(pitch), val: AsVal::conv(val) },
			ChannelAftertouch { val } => ChannelAftertouch { val: AsVal::conv(val) },
		}
	}
}
