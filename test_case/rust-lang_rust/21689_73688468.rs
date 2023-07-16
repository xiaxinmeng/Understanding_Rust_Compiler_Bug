
[03:57:43] <nmatsakis> if you have a defaulted trait
[03:57:48] <nmatsakis> right now you just unconditionally add an entry
[03:57:49] <nmatsakis> that's not quite right
[03:58:08] <nmatsakis> I think we want to match on the (shallowly resolved) self-type
[03:58:16] <nmatsakis> if it is a tyvar, we want to set the ambigiuous flag and bail out
[03:58:40] <nmatsakis> if it is an object, we do not add a default candidate (but see later)
[03:58:45] <nmatsakis> if it is anything else, we add a default candidate
[03:59:12] <nmatsakis> for objects, there exists already assemble_candidates_from_object_ty()
[03:59:18] <nmatsakis> I think that should be extended to cover the builtin bounds case
[03:59:37] <nmatsakis> eventually object types should be arbitrary sums
[03:59:41] <nmatsakis> Foo+Bar whre Foo and Bar are traits
[04:00:05] <nmatsakis> but that seems like the most logical place to put it
[04:01:09] <nmatsakis> we'd have to modify the builtin bounds code then to remove that logic from `builtin_bound` -- which should just return ParameterBuiltin for object types, I think
[04:01:18] <nmatsakis> (I'm going to copy this to the PR for future reference)
