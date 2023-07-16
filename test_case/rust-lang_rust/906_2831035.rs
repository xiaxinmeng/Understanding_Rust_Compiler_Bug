 java
native.rc:
native "cdecl" mod rustrt {
  fn str_buf(str s) -> int;
}

native mod libc = target_libc {
  fn puts(int s) -> ();
}
