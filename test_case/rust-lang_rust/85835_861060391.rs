
let (as, bcs): (Vec<_>, Vec<_>)= iter.map(|(a, b, c)| (a, (b, c))).unzip();
let (bs, cs): (Vec<_>, Vec<_>) = bcs.into_iter().unzip();
