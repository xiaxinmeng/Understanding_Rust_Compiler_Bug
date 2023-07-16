rust
async fn reinvoke_bindings(
    key: &KeyPair,
    target: Recipient<Invocation>,
    provider_id: &str,
    contract_id: &str,
    binding: &str,
    existing_bindings: Vec<(String, HashMap<String, String>)>,
    claims_cache: HashMap<String, Claims<wascap::jwt::Actor>>,
    authorizer: Box<dyn Authorizer>,
) {
...
}
