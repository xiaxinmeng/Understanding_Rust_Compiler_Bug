rust
macro_rules! copy_doc {
    ($(#[$m:meta])* $(cfg $c:tt $i:item)+) => {
        copy_doc!(1 { $(#[$m])* } $(cfg $c $i)+);
    };
    (1 $m:tt $(cfg $c:tt $i:item)+) => {
        copy_doc!(2 $($m cfg $c $i)+);
    };
    (2 $({ $(#[$m:meta])* } cfg $c:tt $i:item)+) => {
        $($(#[$m])* #[cfg $c] $i)+
    };
}
