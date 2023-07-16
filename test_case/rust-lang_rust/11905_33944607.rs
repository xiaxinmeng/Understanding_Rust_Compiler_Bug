
let mut i = 0;
(|| {
   ... i += 1; ...
}).finally(|| {
    use(i);
});
