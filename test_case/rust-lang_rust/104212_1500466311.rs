
for (fresh_trait_pred, eval) in
    self.map.borrow_mut().drain_filter(|_k, eval| eval.from_dfn >= dfn)
{
    debug!(?fresh_trait_pred, ?eval, "on_completion");
}
