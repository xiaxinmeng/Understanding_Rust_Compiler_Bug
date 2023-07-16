 rust
// sequential
for (bi, ai) in bi.mut_iter().zip(ai.iter()) {
    *bi = foo(*ai)
}

// parallel
par_for(bi.mut_iter().zip(ai.iter()), |(bi, ai)| {
    *bi = foo(*ai);
})
