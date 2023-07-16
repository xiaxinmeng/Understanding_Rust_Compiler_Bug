
(|| let bar = &mut *foo; move |b| *bar = b) ;
^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ enclosing statement
 ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~    outer closure
                         ^~~~~~~~~~~~~~~~~    inner closure
