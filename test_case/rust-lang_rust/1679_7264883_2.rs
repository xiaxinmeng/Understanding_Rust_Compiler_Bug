
fn isFlowControl(instr: Instruction) -> option<FlowControl> {
    alt instr {
        x@Jump(*) | x@Return(*) | x@JumpCondition(*) { some(x) }
        _ { none }
    }
}
