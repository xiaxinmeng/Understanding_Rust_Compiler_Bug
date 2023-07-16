
~/code/rust2[eintr] $ ./x86_64-apple-darwin/llvm/Release+Asserts/bin/llvm-mc -filetype=obj ./src/rt/arch/arm/morestack.S -triple=arm-apple-darwin
./src/rt/arch/arm/morestack.S:3:19: error: unexpected token in '.section' directive
.section        .note.GNU-stack, "", %progbits
                         ^
./src/rt/arch/arm/morestack.S:11:7: error: unknown token in expression
.align
      ^
./src/rt/arch/arm/morestack.S:15:1: error: unknown directive
.hidden __morestack
^
./src/rt/arch/arm/morestack.S:19:2: error: unknown directive
 .type __morestack,%function
 ^
Assertion failed: (TargetStreamer), function getTargetStreamer, file ../../../../../../src/llvm/include/llvm/MC/MCStreamer.h, line 173.
0  llvm-mc                  0x000000010f9e2ae8 llvm::sys::PrintStackTrace(__sFILE*) + 40
1  llvm-mc                  0x000000010f9e2fe4 SignalHandler(int) + 548
2  libsystem_platform.dylib 0x00007fff87b005aa _sigtramp + 26
3  libsystem_platform.dylib 000000000000000000 _sigtramp + 2018507376
4  llvm-mc                  0x000000010f9e2da6 abort + 22
5  llvm-mc                  0x000000010f9e2d81 __assert_rtn + 81
6  llvm-mc                  0x000000010f8aa302 (anonymous namespace)::ARMAsmParser::ParseDirective(llvm::AsmToken) + 10834
7  llvm-mc                  0x000000010f92b4e3 (anonymous namespace)::AsmParser::parseStatement((anonymous namespace)::ParseStatementInfo&) + 2083
8  llvm-mc                  0x000000010f927407 (anonymous namespace)::AsmParser::Run(bool, bool) + 439
9  llvm-mc                  0x000000010f848053 main + 4275
10 libdyld.dylib            0x00007fff885be5fd start + 1
Stack dump:
0.      Program arguments: ./x86_64-apple-darwin/llvm/Release+Asserts/bin/llvm-mc -filetype=obj ./src/rt/arch/arm/morestack.S -triple=arm-apple-darwin 
zsh: illegal hardware instruction (core dumped)  ./x86_64-apple-darwin/llvm/Release+Asserts/bin/llvm-mc -filetype=obj  
