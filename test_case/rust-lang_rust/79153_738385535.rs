
 38   /// When an error signal (such as SIGABRT or SIGSEGV) is delivered to the
 39   /// process, print a stack trace and then exit.
 40   /// Print a stack trace if a fatal signal occurs.
 41   /// \param Argv0 the current binary name, used to find the symbolizer
 42   ///        relative to the current binary before searching $PATH; can be
 43   ///        StringRef(), in which case we will only search $PATH.
 44   /// \param DisableCrashReporting if \c true, disable the normal crash
 45   ///        reporting mechanisms on the underlying operating system.
 46   void PrintStackTraceOnErrorSignal(StringRef Argv0,
 47                                     bool DisableCrashReporting = false);
