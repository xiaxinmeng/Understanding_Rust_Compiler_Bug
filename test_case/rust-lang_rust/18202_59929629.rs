
/// Move all the elements in the tail portion of this structure into a new structure
/// For sorted maps/sets this would be a key. Not really appropriate for hashmap or priorityqueue.
fn split(&mut self, at: &uint) -> Self;
