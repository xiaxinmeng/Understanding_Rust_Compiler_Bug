rust
pub struct MarginTxBuilder {
    rpc: Arc<dyn SolanaRpcClient>,
    ///
    pub ix: MarginIxBuilder,
    config_ix: MarginConfigIxBuilder,
    ///
    pub signer: Option<Keypair>,
    is_liquidator: bool,
}
