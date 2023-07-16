
struct RcString { ... }

Vec<RcString> known_strings;

fn reclaim_memory_used_by_keys(map: &mut HashMap<RcString, Something>) {
    for s in &known_strings {
        if let Entry::Occupied(e) = Entrymap.entry(s) {
            e.replace_key();
        }
    }
}
