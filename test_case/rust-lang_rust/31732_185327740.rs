
<Copy the existing text from the issue for the summary>

I tried this code:

let set1: HashSet<u8, FnvHasher> = [1,2,3].collect();
let set2: HashSet<u8, SipHasher> = [2,4,6].collect();
let middle: Vec<u8> = set1.intersect(&set2).collect();
println!("{:?}", middle);

I expected to see this happen: 

The code compiles and `4` is printed.

Instead, this happened:

<copy in the exact compliler error message>

## Meta

<whatever my nightly is>
