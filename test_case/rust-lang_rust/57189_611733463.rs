
pub struct EventHandlingStruct;

impl EventHandlingStruct {
	fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) {}
}

pub struct Gui {
	current_note: u8
}

impl Gui {
	fn connect_signals(&mut self, builder: &EventHandlingStruct) {
		builder.connect_changed(move |_| {

			// bunch of code
			
			// this is the problem line that extends the lifetime of the closure too much
			dbg!(self.current_note);

			// bunch of code
		});
	}
}
