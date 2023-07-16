
"äbc".as_char_str().slice_from(1) == "bc"
"äbc".as_char_str().slice_to(1) == "ä"
"äbc".as_char_str().reverse_slice_from(1) == "äb"
"äbc".as_char_str().reverse_slice_to(1) == "c"

"äbc".as_utf8_str().slice_from(2) == "bc"
"äbc".as_utf8_str().slice_to(2) == "ä"
"äbc".as_utf8_str().reverse_slice_from(1) == "äb"
"äbc".as_utf8_str().reverse_slice_to(1) == "c"
