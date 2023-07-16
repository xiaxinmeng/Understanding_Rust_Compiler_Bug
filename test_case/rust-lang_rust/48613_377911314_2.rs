
Call graph:
    2694 Thread_7720   DispatchQueue_1: com.apple.main-thread  (serial)
      2694 start  (in libdyld.dylib) + 1  [0x7fff6096f015]
        2694 0x0
          2694 _sigtramp  (in libsystem_platform.dylib) + 26  [0x7fff60c7df5a]
            2694 std::sys::unix::stack_overflow::imp::signal_handler::h90fb82b6c0c85c05 (.llvm.7884192080853486565)  (in deadlock) + 120  [0x108d07c78]

