
[05:25:24] <nmatsakis> huon: no, but it's a tricky problem
[05:25:26] <nmatsakis> I think that falls under
[05:25:37] <nmatsakis> "method resolution should keep records near-misses"
[05:25:42] <nmatsakis> there've been a couple of other cases where I wanted that
[05:25:49] <nmatsakis> it does have a list of candidates
[05:25:58] <nmatsakis> so it's certainly plausible for it to do a better job
[05:26:03] <nmatsakis> and it could definitely keep track of
[05:26:06] <nmatsakis> candidates that it threw out
[05:26:11] <nmatsakis> beacuse of "winnowing" (evaluating the predicates)
[05:26:21] <nmatsakis> I like how gcc does this
[05:26:22] <huon> would effectively moving the bounds to where clauses on each method be invalid?
[05:26:30] <nmatsakis> well, that would be a workaround
[05:26:38] <nmatsakis> it might invalidate some coherence rules
[05:27:00] <nmatsakis> gcc often, when things don't resolve,
[05:27:04] <nmatsakis> gives you a list of all the things it considered
[05:27:07] <nmatsakis> and tells you why each one failed
[05:27:09] <nmatsakis> we could certainly do that too
[05:27:13] <nmatsakis> perhaps with some screening
[05:27:22] <nmatsakis> because right now there can be a LOT of obviously wrong candidates
[05:27:31] <nmatsakis> the other thing I want to do which is semi-related
[05:27:35] <nmatsakis> is to find traits that are not imported
[05:27:37] <nmatsakis> but which define the method in question
[05:27:40] <nmatsakis> and mention those
[05:27:40] <huon> yes!
[05:27:48] <huon> that's probably the biggest trait-related trip-up
