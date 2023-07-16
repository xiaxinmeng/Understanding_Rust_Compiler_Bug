rust
while let Some(work_item) = work_lists.iter_mut().find_map(|h| h.pop()) {
    ...
}
