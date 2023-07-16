
> macro_rules! foo { ($x:pat | $x:pat) => {} } // emit suggestion
> macro_rules! bar { ($(x:pat)+ | $($x:pat)+) => {} } // emit suggestion
> macro_rules! baz { ($x:pat2015 | $x:pat2015) => {} } // should be ok
> macro_rules! qux { ($x:pat2015 | $x:pat) => {} } // should be ok
> macro_rules! ogg { ($x:pat | $x:pat2015) => {} } // emit suggestion
> 