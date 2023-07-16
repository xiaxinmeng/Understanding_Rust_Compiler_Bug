
> rustc -C debuginfo=2 .\src\main.rs
> .\main.exe
thread '<unnamed>' panicked at 'malice is panicking!', .\src\main.rs:16:5
stack backtrace:
   0: std::panicking::begin_panic<str>
             at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e\library\std\src\panicking.rs:616
   1: main::malice
             at .\src\main.rs:16
   2: main::bob
             at .\src\main.rs:12
   3: main::alice::closure$0
             at .\src\main.rs:7
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'main' panicked at 'malice is panicking!', .\src\main.rs:16:5
stack backtrace:
   0: std::panicking::begin_panic<str>
             at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e\library\std\src\panicking.rs:616
   1: main::malice
             at .\src\main.rs:16
   2: main::bob
             at .\src\main.rs:12
   3: main::main
             at .\src\main.rs:23
   4: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
             at /rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e\library\core\src\ops\function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
