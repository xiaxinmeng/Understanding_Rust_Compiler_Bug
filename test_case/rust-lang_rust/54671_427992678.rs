
    #![no_core] =>
        extern_prelude += []

    #![no_std] =>
        extern_prelude += [core]

    default in 2015 => // default means the absence of #![no_core] && #![no_std]
        extern_prelude += [core, std]

    default in 2018 => // default means the absence of #![no_core] && #![no_std]
        extern_prelude += [core, std, meta]

    extern crate foo; =>
        extern_prelude += [foo]

    extern crate foo as bar; =>
        extern_prelude += [bar]
