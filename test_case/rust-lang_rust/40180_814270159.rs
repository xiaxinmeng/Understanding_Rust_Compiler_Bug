cpp
if (F.getCallingConv() == CallingConv::X86_INTR) {
    // IA Interrupt passes frame (1st parameter) by value in the stack.
    if (Idx == 1)
        Flags.setByVal();
}
