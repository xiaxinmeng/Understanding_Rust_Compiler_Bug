
<anon>:6:5: 6:13 error: mismatched types:
 expected `core::option::Option<T>`,
    found `core::option::Option<<core::slice::Iter<'a, T> as core::iter::Iterator>::Item>`
(expected type parameter,
    found associated type) [E0308]
<anon>:6     v.next()
             ^~~~~~~~
error: aborting due to previous error
playpen: application terminated with error code 101
