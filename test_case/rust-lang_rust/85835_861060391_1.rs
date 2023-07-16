
let (as, (bs, cs)): (Vec<_>, (Vec<_>, Vec<_>)) = iter.map(|(a, b, c)| (a, (b, c))).unzip();
