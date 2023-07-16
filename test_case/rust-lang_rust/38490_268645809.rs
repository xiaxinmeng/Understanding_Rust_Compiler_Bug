
// One module
struct Private;

pub macro m() {
    Private // <- ok, the name is resolved, path privacy check passes
}

// Other module
let secret = m!(); // <- object of a private type is walking freely outside of its module
