
// Chain such that we
// 1. Create a future computing some partial result
// 2. wait for a result 
// 3. then recover to a new future in case of error, 
// 4. then try its awaited result. 
async fn await_chain() -> Result<usize, Error> {
    // Mandatory delimiters
    let _ = await(await(partial_computation()).unwrap_or_else(or_recover))?
    // Useful precedence requires paranthesis nesting afterall
    let _ = await { await partial_computation() }.unwrap_or_else(or_recover)?;
    // Obivious precendence may do slightly better, but I think confusing left-right-jumps after all.
    let _ = await? (await partial_computation()).unwrap_or_else(or_recover);
    // Post-fix
    let _ = partial_computation()(await).unwrap_or_else(or_recover)(await)?;
}
