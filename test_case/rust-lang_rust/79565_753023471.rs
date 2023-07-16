shell
$ rustc fixed/72348.rs -Zinstrument-coverage -Zsymbol-mangling-version=legacy
warning: -Z instrument-coverage requires symbol mangling version `v0`, but `-Z symbol-mangling-version=legacy` was specified
