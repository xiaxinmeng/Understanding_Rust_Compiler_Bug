
<anon>:11:1: 11:39 error: conflicting implementations for trait `FnOnce` [E0119]
<anon>:11 impl<A> FnOnce<A> for Box<FnBox<A>> {}
          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:11:1: 11:39 help: see the detailed explanation for E0119
<anon>:13:1: 13:55 note: note conflicting implementation here
<anon>:13 impl<F: FnOnce<A> + ?Sized, A> FnOnce<A> for Box<F> {}
          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
