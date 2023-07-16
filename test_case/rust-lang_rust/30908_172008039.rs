
// uninitialized
size_t str_size;
// initialize str_size by copying 4 bytes from some localion
memcpy (&str_size, syms_view.data + syms_size, 4); 
