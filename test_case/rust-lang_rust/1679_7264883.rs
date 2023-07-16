
enum Instruction {
    BinExp(...),
    Load(...),
    Store(...),
    Jump(...),
    Call(...),
    Return(...)
}   

fn usedVariables(inst: Instruction) -> ~[Variable] {
    alt inst {
        BinExp(_, t1, t2)   { ~[t1, t2] }
        Store(_, t)         { ~[t] }
        ...
        ... more similar cases follow ...
        ... 
        // All other instructions don't use any variables!
        _                   { ~[] }
    }   
}   

fn successorLines(inst: Instruction, linenr: uint) -> ~[uint] {
    alt inst {
        Jump(lbl)   { ~[lookupLabel(lbl)] }
        Return(_)   { ~[] }
        // All other instructions don't alter flow control.
        _           { ~[linenr + 1] }
    }   
}   
