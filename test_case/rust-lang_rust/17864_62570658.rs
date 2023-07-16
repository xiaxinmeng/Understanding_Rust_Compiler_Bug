
[08:12:49] <nmatsakis> pcwalton: do we want to impose a rule that you cannot impl Copy if you impl Drop? (and do you impose such a rule?)
[08:12:56] <nmatsakis> pcwalton: it's not obvious to me that this restriction is NECESSARY
[08:13:16] <nmatsakis> pcwalton: though perhaps at minimum we shouldn't lint about it I guess
[08:13:24] <nmatsakis> pcwalton: but basically r+
[08:14:02] <nmatsakis> pcwalton: I read the changes to middle::traits and middle:coherence, I assume the rest is fallout. <nag>it'd be helpful to segregate that into its own commit.</nag>
[08:14:04] <nmatsakis> I'll leave a comment
