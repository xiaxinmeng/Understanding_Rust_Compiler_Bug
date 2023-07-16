
    (maybe you meant: _rust_uv_strerror)
  "_strlen", referenced from:
      tm_to_rust_tm(tm*, rust_tm*, int, char const*, int)in rust_builtin.o
      _rust_localtime in rust_builtin.o
      load_env(int, char**)in rust_env.o
      update_entry(mod_entry const*, void*)in rust_log.o
      update_log_settings(void*, char*)in rust_log.o
      append_string(char*, char const*, ...)in rust_log.o
      _remove_char in linenoise.o
      ...
  "_strncmp", referenced from:
      update_entry(mod_entry const*, void*)in rust_log.o
  "_strncpy", referenced from:
      load_env(int, char**)in rust_env.o
      copy_string(char*, char const*, unsigned long)in rust_log.o
      update_log_settings(void*, char*)in rust_log.o
      _linenoise in linenoise.o
  "_strstr", referenced from:
      _linenoise in linenoise.o


  "_uv_write", referenced from:
      _rust_uv_write in rust_uv.o
     (maybe you meant: _rust_uv_helper_uv_write_t_size, _rust_uv_write )
  "_vsnprintf", referenced from:
      append_string(char*, char const*, ...)in rust_log.o
      rust_log::log(rust_task*, unsigned int, char const*, ...)in rust_log.o
      rust_kernel::fatal(char const*, ...)in rust_kernel.o
      rust_kernel::log(unsigned int, char const*, ...)in rust_kernel.o
      _fd_printf in linenoise.o
  "_write", referenced from:
      _fd_printf in linenoise.o
      _insert_char in linenoise.o
      _refreshLine in linenoise.o
     (maybe you meant: _rust_uv_helper_uv_write_t_size, _rust_uv_write , _rust_uv_get_stream_handle_from_write_req )
ld: symbol(s) not found for architecture armv7
collect2: ld returned 1 exit status
make: *** [rt/arm-apple-darwin/librustrt.dylib] Error 1
