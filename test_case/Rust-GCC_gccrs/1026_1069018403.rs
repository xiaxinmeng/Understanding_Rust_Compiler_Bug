c++
../../gcc/rust/rust-session-manager.cc:1011:19: warning: format ‘%s’ expects argument of type ‘char*’, but argument 3 has type ‘std::string’ {aka ‘std::__cxx11::basic_string<char>’} [-Wformat=]
 1011 |       rust_debug ("had to implicitly enable feature '%s'!", feature);
      |                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
../../gcc/rust/rust-diagnostics.h:143:54: note: in definition of macro ‘rust_debug’
  143 | #define rust_debug(...) rust_debug_loc (Location (), __VA_ARGS__)
      |                                                      ^~~~~~~~~~~
../../gcc/rust/rust-session-manager.cc:1011:55: note: format string is defined here
 1011 |       rust_debug ("had to implicitly enable feature '%s'!", feature);
      |                                                      ~^
      |                                                       |
      |                                                       char*
