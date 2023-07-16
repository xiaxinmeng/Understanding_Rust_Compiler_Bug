
> > 
> > 
> > This code does not compile for the same reason. Unsafe fn's are still fns too, why we haven't implicit casting in this situation?
> 
> This leads to possible unsafety in our code, and all which comes with it. `const ` fn casting on otherside don't brings any unsafety, so is allowed, const fn must not have any side effects only, it is compatible with fn contract.

I think you're missing the point. With implicit coercions, the type of `x` would be `unsafe fn()`, not `fn()`. There's nothing about that which leads to possible unsafety. Generally, `const fn()` can be coerced to `fn()` and `fn()` can be coerced to `unsafe fn()`. It just doesn't happen automatically, which is why changing `const fn foo()` to coerce into `const fn()` rather than `fn()` implicitly is a breaking change.

> 