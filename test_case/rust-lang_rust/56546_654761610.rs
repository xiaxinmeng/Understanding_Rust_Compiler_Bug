rust
trait FunctionalitySet1 {
	/// Elaborate documentation...
	fn do_something(&mut self) {
		// ...
	}

	// ... more methods, all documented here within the trait
}

trait FunctionalitySet2 {
	/// Elaborate documentation...
	fn do_fancy_thing(&mut self) {
		// ...
	}

	// ... more methods, all documented here within the trait
}

trait FunctionalitySet3 {
	/// Elaborate documentation...
	fn do_something_exotic(&mut self) {
		// ...
	}

	// ... more methods, all documented here within the trait
}

// Depending on which capability sets each hardware device supports, we can just implement the
// corresponding traits.

struct DeviceA { /* ... */ }
impl FunctionalitySet1 for DeviceA {}
impl FunctionalitySet2 for DeviceA {}

struct DeviceB { /* ... */ }
impl FunctionalitySet3 for DeviceB {}

struct DeviceC { /* ... */ }
impl FunctionalitySet1 for DeviceC {}
impl FunctionalitySet3 for DeviceC {}
