 rust

macro_rules! cfg_match {
    (_ => $b:block) => {
        $b
    };
    ($cfg:meta => $b:block $($t:tt)*) => {
        (
            #[cfg($cfg)] $b,
            #[cfg(not($cfg))] (cfg_match!($($t)*)),
        ).0
    };
    () => {
        {
            panic!("Conditional code not supported under this configuration");
        }
    }
}

let x = cfg_match! {
    unix => { foo() }
    windows => { bar() }
};
