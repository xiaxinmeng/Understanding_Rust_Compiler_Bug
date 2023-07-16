rust
let checked_cache = cache.as_ref().filter(|e| e.is_valid());
let content = match checked_cache {
	Some(e) => &e.content,
	None => {
	    cache = None;
	    &checked_cache.get_or_insert_with(build).content
	}
};

