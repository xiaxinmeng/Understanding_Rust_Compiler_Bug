
12:22 < brson> I'm trying to decide what to do about the argument order to vec::foldl - it has a different order than iter::foldl and list::foldl, but it
               arguably has the most natural order for 'foldl'
12:22 < brson> it takes the list second
12:23 < brson> soliciting opinions
12:24 <@graydon> I think usually we make the functions come last so we can support foo(x) { | x, y | ... } form
12:25 < brson> yes, I agree
12:25 < brson> but vec::foldl looks like fn foldl<T: copy, U>(z: T, v: [const U], p: fn(T, U) -> T) -> T
12:25 < brson> and we usually also make the vector come first
12:26 <@graydon> ah
12:26 <@graydon> in the case of a fold I think this is to match the binary-operator-ness of the function
12:26 < brson> yeah
12:26 <@graydon> list goes the other way?
12:26 < brson> list goes the other way
12:27 <@graydon> weird. hmm. fold is a rare beast in this respect, but I think I _slightly_ prefer what vec is doing over what list is doing.
12:27 < brson> iter also goes the other way, but it's because of the self argument, and since that's implicit maybe it doesn't matter
12:27 <@graydon> not a strong opinion
12:28 < brson> oh, actually no. iter has an actual function with an argument called 'self', so both list and iter do it backwards
