
static some: Option<~uint> = None; // should be copyable or movable
static some: Option<~uint> = Some(~1); // should be neither copyable nor movable
