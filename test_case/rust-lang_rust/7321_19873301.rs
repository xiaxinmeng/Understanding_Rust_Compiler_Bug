
let v = [1,2,3,4,5];
v.iter().advance(|&i| { println(i.to_str()); true })

// ...

let v = [1,2,3];
for v.iter().advance |&i| {
    println(i.to_str());
}
