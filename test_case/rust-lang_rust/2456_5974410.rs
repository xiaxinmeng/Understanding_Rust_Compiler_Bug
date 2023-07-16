
fn chmod(filepath: str, mode: i32) -> i32 {
   import libc::types::os::arch::posix88::mode_t;

   ret str::as_c_str(filepath, {|fp| libc::chmod(fp, mode as mode_t) });
}

fn cp(src: str, dest: str) {
   // What I'd like to do is something like so,
   // but where's the stat function?
   //
   //let st = libc::stat(src);
   //let mm = st.mode;

   let mm = 493_i32; /* octal 755 */
   os::copy_file(src, dest);
   chmod(dest, mm);
}

fn main() {
   cp("./dir/a", "./dir2/a");
}
