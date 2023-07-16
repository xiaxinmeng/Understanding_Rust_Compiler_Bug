
do draw_each(screen, map) |position : map::Position, _ : map::Tile| {
    if player.sees(position) {
        // This pointer must provably be valid in the caller's stack frame, which means that it must be in a region
        // that is known to the caller.
        let thing = &*fog;
        Some(thing)
    } else {
        None
    }
}

// The draw each signature
fn draw_each(screen : &video::Surface, map : &mut map::Map,
                       f : fn(position : map::Position, tile : map::Tile) -> Option<&video::Surface>) { ... }
