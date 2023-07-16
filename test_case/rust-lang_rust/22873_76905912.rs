
C:/bot/slave/auto-win-64-nopt-t/build/src/librustdoc\html\render.rs:500:36: 500:57 error: mismatched types:
 expected `&std::old_path::windows::Path`,
    found `&std::path::PathBuf`
(expected struct `std::old_path::windows::Path`,
    found struct `std::path::PathBuf`) [E0308]
C:/bot/slave/auto-win-64-nopt-t/build/src/librustdoc\html\render.rs:500     let _lock = ::flock::Lock::new(&cx.dst.join(".lock"));

