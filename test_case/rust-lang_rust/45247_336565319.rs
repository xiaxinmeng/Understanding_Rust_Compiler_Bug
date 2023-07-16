rust
#[cfg(stage0)]
macro_rules! conditional {
    (stage0 => { $($stage0:tt)* } stageN => { $($stageN:tt)* }) => $($stage0)*
}

#[cfg(not(stage0))]
macro_rules! conditional {
    (stage0 => { $($stage0:tt)* } stageN => { $($stageN:tt)* }) => $($stageN)*
}
