rust
// not panic-free
let _ = Checked::new(Duration::new(s, ns));

// weird and not panic-free
let _ = Checked(Some(Duration::new(s, ns)));

// :( less important words up front
let _ = Checked::new_duration(s, ns);

// ???
let _ = Checked::now();
let _ = Checked::<Instant>::now();
let _ = Checked::now_instant();
let _ = Checked::instant_now();

// fine...
// except people would write Checked::new(s, ns)
// which is confusing and breaks when we later add `new` method for another type
let _ = Checked::<Duration>::new(s, ns);

// checked_ prefix implies it returns Option, not Checked<Duration>
let _ = Duration::checked_new(s, ns);

// not bad...
let _ = Duration::new_checked(s, ns);

// ... but for Instant?
let _ = Instant::now_checked(); // ???

// easytime
let _ = Duration::new(s, ns);
let _ = Instant::now();
