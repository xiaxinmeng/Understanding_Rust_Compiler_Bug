
// Code fragment I actually wrote today, before discovering insert_entry is nightly only
fn map_write_if_greater<K,T>(map:HashMap<K,T>, key:K, v:T) where K:Eq+std::hash::Hash, T:PartialEq+PartialOrd {
	let entry = map.entry(key);
	if match entry { Entry::Vacant(_) => true, Entry::Occupied(v2) => v>*v2.get()} {
		entry.insert_entry(v); // Avoid second hash lookup
	}
}
