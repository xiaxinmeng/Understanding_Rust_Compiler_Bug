
2019-11-13T18:34:56.0487497Z let x: fn(S) = foo; // OK, coerces
2019-11-13T18:34:56.0487757Z 
2019-11-13T18:34:56.0487757Z 
2019-11-13T18:34:56.0487917Z The reason that this matter is that the type `fn(S)` is not specific to
2019-11-13T18:34:56.0488443Z any particular function: it's a function _pointer_. So calling `x()` results
2019-11-13T18:34:56.0488653Z in a virtual call, whereas `foo()` is statically dispatched, because the type
2019-11-13T18:34:56.0488816Z of `foo` tells us precisely what function is being called.
2019-11-13T18:34:56.0488959Z 
2019-11-13T18:34:56.0489360Z As noted above, coercions mean that most code doesn't have to be
2019-11-13T18:34:56.0489550Z concerned with this distinction. However, you can tell the difference
2019-11-13T18:34:56.0489810Z when using **transmute** to convert a fn item into a fn pointer.
2019-11-13T18:34:56.0490095Z This is sometimes done as part of an FFI:
2019-11-13T18:34:56.0490212Z 
2019-11-13T18:34:56.0490358Z 