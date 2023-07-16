rust
enum SubcommandKind {
    Build,
    Check,
    ...
}
impl FromStr for SubcommandKind {...}
impl SubcommandKind {
    fn add_extra_opts(&self, opts: &mut Options) { /* flags.rs lines 279-338 */}
    fn add_extra_help(&self, subcommand_help: &mut String) { /* flags.rs lines 397-539 */ }
    fn build_subcommand(&self, paths: Vec<PathBuf>, matches: &Matches) -> Result<Subcommand> {
        /* flags.rs lines 550-624 */
    }
}
