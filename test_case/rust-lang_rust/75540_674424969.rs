
iterator -> | 1 2 3 4 5
iterator.skip_while (!= 2) -> 1 | 2 3 4 5
iterator.skip_while.peekable.take_until (== 3) -> 1 2 | 3 4 5 (note we only iterated 2 here)
The next time we iterate
iterator -> 1 2 | 3 4 5
What we wanted
iterator -> 1 | 2 3 4 5
What should be done
iterator.peekable.skip_while.take_until (not possible right now)
