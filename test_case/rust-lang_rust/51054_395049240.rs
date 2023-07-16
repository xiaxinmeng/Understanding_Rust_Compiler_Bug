
<njn> so I am wondering why `parent_map` (and the entire scope tree) is hashed at all,
      because it seems to me to be "derived data", in the sense that it'll only change if
      the HIR above it also changes
<Zoxc> I'm not sure why it's hashed, it seems redundant to me if it is treated as an input
       along with the HIR
<njn> Right. So I'm wondering if there's a clear notion of what should be hashed and what
      shouldn't
<Zoxc> We probably want to introduce a query kind which does not recheck the result for
       freshness for these cases. That would avoid the hashing
<njn> what is the definition of "these cases" -- derived data?
<Zoxc> No. It would be queries D, where query D uses queries A, and the users of D also
       use A.
<Zoxc> So if such a query D is executed, comparing its result to the previous session and
       marking it up to date will have no affect on its users since they use the same
       queries A and will execute anyway.
<njn> how does one identify these cases? Does it require domain expertise on a
      case-by-case basis?
<Zoxc> It probably does
<njn> :(
<njn> I guess if a handful of structures account for most of the hashing cost, that'll make
      it easier
<Zoxc> You could also just change one query at a time and benchmark things, starting
       with those with the largest query results
