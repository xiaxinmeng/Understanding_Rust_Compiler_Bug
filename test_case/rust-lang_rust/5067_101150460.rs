
<anon>:2:17: 2:36 error: `$e1:expr` is followed by a sequence repetition, which is not allowed for `expr` fragments
<anon>:2     (a $e1:expr $($(, a $e2:expr)*)*) => ([$e1 $($(, $e2)*)*]);
                         ^~~~~~~~~~~~~~~~~~~
