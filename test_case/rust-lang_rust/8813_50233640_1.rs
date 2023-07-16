
<anon>:19:20: 19:29 error: use of moved value: `something`
<anon>:19     mut_object_bar(something);
                             ^~~~~~~~~
<anon>:18:20: 18:29 note: `something` moved here because it has type `&mut TraitA`, which is non-copyable (perhaps you meant to use clone()?)
<anon>:18     mut_object_bar(something);
                             ^~~~~~~~~
<anon>:24:16: 24:25 error: use of moved value: `something`
<anon>:24     object_bar(something);
                         ^~~~~~~~~
<anon>:23:16: 23:25 note: `something` moved here because it has type `&TraitA`, which is moved by default (use `copy` to override)
<anon>:23     object_bar(something);
                         ^~~~~~~~~
error: aborting due to 2 previous errors
playpen: application terminated with error code 101
