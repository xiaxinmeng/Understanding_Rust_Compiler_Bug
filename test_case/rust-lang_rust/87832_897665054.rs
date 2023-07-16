rust
match foo { // (1) coverage/debugger step on entry to match expression for executing `foo`
    Arm1 => { // (2) debugger steps here when this match arm's condition is evaluated
        body; // coverage/debugger step if body executes
    } // no coverage here (presuming nothing drops in this arm)
    Arm2 => { // (3) debugger steps here when this match arm's condition is evaluated. Note that this may be in a different order depending on optimizations compared to Arm1 (this is OK).
        body; // (4) coverage/debugger step if body executes
    }
}
