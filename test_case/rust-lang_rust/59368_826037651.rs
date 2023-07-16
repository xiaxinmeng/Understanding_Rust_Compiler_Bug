rust
#[macro_use]
macro_rules! export {
    (
        $(#[$m:meta])*
        macro_rules! $name:ident {
            $(($($token:tt)*) => ($($expansion:tt)*));* $(;)?
        }
    ) => (
        $(#[$m])*
        #[doc(hidden)]
        #[cfg(not(nightly))]
        #[macro_export]
        macro_rules! $name {
            $(
                ($($token)*) => ($($expansion)*)
            );*
        }

        $(#[$m])*
        #[cfg(not(nightly))]
        pub use $name;

        $(#[$m])*
        #[cfg(nightly)]
        pub macro $name {
            $(
                ($($token)*) => ($($expansion)*)
            ),*
        }
    )
}
