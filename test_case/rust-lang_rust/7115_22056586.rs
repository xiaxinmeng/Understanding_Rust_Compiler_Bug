
< Luqman> We seem to be running into a weird codegen issue upon trying to upgrade the llvm build we use with rust. we noticed this because one of our tests were failing upon trying to upgrade
< Luqman> looking at the llvm IR & assembly output it seems ok (if a bit verbose) at first https://gist.github.com/luqmana/6146443
< Luqman> until you notice that there's one instruction in the assembly output, movb %ah, (%r8)
< Luqman> which is not a valid x86 instruction as i understand it, and disassembling the binary it becomes mov %spl, (%r8)
< Luqman> which, while valid, is semantically wrong
...
< roxfan> Luqman: yep, looks like a bug to me
< roxfan> assembler shouldn't accept it either
