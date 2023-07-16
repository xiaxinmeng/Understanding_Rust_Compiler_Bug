
let tmps = ~[];
for vec::each(args) |arg| {
    tmps.push(copy *arg);
}
