
> // X86-32 C return-value convention.
> def RetCC_X86_32_C : CallingConv<[
>   // The X86-32 calling convention returns FP values in FP0, unless marked
>   // with "inreg" (used here to distinguish one kind of reg from another,
>   // weirdly; this is really the sse-regparm calling convention) in which
>   // case they use XMM0, otherwise it is the same as the common X86 calling
>   // conv.
> 