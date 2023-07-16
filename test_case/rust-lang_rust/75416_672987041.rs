rust
        // FIXME(richkadel): Consider changing the implementation from using "Call" terminators to
        // using a custom `Statement`. The current implementation leads to slightly less optimal
        // LLVM IR. Using a new `Statement` type would have a few additional benefits as well:
        //
        // 1. One problem with using a Call `Terminator` is a `Terminator` requires its own
        //    BasicBlock, but as it turns out, LLVM's `instrprof.increment` intrinsic is converted
        //    into a set of inline statements that don't actually perform a function call and don't
        //    need to unwind or branch. However, the existence of the BasicBlock in the Rust MIR CFG
        //    results the generation of an additional BasicBlock label, and an unnecessary `br`anch,
        //    from the statement just after the increment statements to the label for the next
        //    block, even though the first statement of the next block is sequentionally next
        //    anyway. Here's an LLVM IR snippet to demonstrate this. The example function has no
        //    statements:
        //
        //    # No instrumentation:
        //    