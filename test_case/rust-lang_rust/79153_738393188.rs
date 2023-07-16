
1304 pub fn main() -> ! {
1305     let start = Instant::now();
1306     init_rustc_env_logger();
1307     let mut callbacks = TimePassesCallbacks::default();
1308     install_ice_hook();
1309     let exit_code = catch_with_exit_code(|| {
1310         let args = env::args_os()
