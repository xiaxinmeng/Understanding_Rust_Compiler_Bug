rust
const fn concat<T>(this: [T; 4], that: [T; 4]) -> [T; 8] {
    let [a, b, c, d] = this;
    let [e, f, g, h] = that;
    [a, b, c, d, e, f, g, h]
}
