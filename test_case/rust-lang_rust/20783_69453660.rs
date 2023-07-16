
[04:04:59] <nmatsakis> Manishearth: so the place to start is librustc/middle/traits/error_reporting.rs
[04:05:23] <nmatsakis> Manishearth: when it finds that a TraitPredicate is Unimplemented, I would then go and fetch the attributes from the trait's def-id
[04:05:37] <nmatsakis> Manishearth: (`ty::has_attr` or something similar)
[04:05:42] <nmatsakis> Manishearth: and then extract the format string
[04:05:53] <nmatsakis> Manishearth: prepare the arguments by converting each type argument in the trait ref to a string
[04:06:04] <nmatsakis> Manishearth: and invoke the format_args! logic (you can't use the macro itself)
[04:06:25] <nmatsakis> Manishearth: at least that was my plan, when I write it out doesn't sound QUITE so trivial, in that there's a bit of familiarity with compiler data structures involved
