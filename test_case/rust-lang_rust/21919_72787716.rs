
rustc: i686-pc-windows-gnu/stage1/bin/rustlib/i686-pc-windows-gnu/lib/libstd
C:\bot\slave\auto-win-32-nopt-t\build\src\libstd\lib.rs:117:12: 117:26 warning: feature is deprecated and will only be available for a limited time, please rewrite code that relies on it
C:\bot\slave\auto-win-32-nopt-t\build\src\libstd\lib.rs:117 #![feature(old_impl_check)]
                                                                       ^~~~~~~~~~~~~~
C:\bot\slave\auto-win-32-nopt-t\build\src\libstd\sys\windows\os.rs:194:10: 194:14 warning: derive(Show) is deprecated in favor of derive(Debug)
C:\bot\slave\auto-win-32-nopt-t\build\src\libstd\sys\windows\os.rs:194 #[derive(Show)]
                                                                                ^~~~
C:\bot\slave\auto-win-32-nopt-t\build\src\libstd\lib.rs:117:12: 117:26 warning: feature is deprecated and will only be available for a limited time, please rewrite code that relies on it
C:\bot\slave\auto-win-32-nopt-t\build\src\libstd\lib.rs:117 #![feature(old_impl_check)]
                                                                       ^~~~~~~~~~~~~~
C:\bot\slave\auto-win-32-nopt-t\build\src\libstd\old_io\mod.rs:1613:10: 1613:11 warning: the type parameter `T` is not constrained by the impl trait, self type, or predicates
C:\bot\slave\auto-win-32-nopt-t\build\src\libstd\old_io\mod.rs:1613 impl<'a, T, A: ?Sized + Acceptor<T>> Iterator for IncomingConnections<'a, A> {
                                                                             ^
/c/bot/slave/auto-win-32-nopt-t/build/mk/target.mk:165: recipe for target 'i686-pc-windows-gnu/stage1/bin/rustlib/i686-pc-windows-gnu/lib/stamp.std' failed

This application has requested the Runtime to terminate it in an unusual way.
Please contact the application's support team for more information.
Assertion failed!

Program: C:\bot\slave\auto-win-32-nopt-t\build\obj\i686-pc-windows-gnu\stage1\bin\rustc.exe
File: C:/bot/slave/auto-win-32-nopt-t/build/src/llvm/lib/Analysis/CodeMetrics.cpp, Line 101

Expression: I->getParent()->getParent() == F && "Found assumption for the wrong function!"
make: *** [i686-pc-windows-gnu/stage1/bin/rustlib/i686-pc-windows-gnu/lib/stamp.std] Error 3
program finished with exit code 2
