 rust
let alignment = get_alignment(val);
let offset = if likely(alignment == word_align) {
    calc_offset(initial_offset, word_align)
} else {
    calc_offset(initial_offset, alignment)
};
