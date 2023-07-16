
if edition_2015 =>
    extern_prelude += []

if edition_2018 =>
    #![no_core] =>
        extern_prelude += []

    #![no_std] =>
        extern_prelude += [core]

    default => // default means the absence of #![no_core] && #![no_std]
        extern_prelude += [core, std, meta]

    extern crate foo; =>
        extern_prelude += [foo]

    extern crate foo as bar; =>
        extern_prelude += [bar]
