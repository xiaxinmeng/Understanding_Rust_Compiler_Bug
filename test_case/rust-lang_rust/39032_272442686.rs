
#[cfg(stage0)]
macro_rules! attribute {
    ($($ts: tt)*) => {
        $($ts)*
    }
}
#[cfg(not(stage0))]
#[allow_internal_unstable]
#[other_stuff]
macro_rules! attribute {
    ($($ts: tt)*) => {
        #[attribute1] #[attribute2] $($ts)*
    }
}
