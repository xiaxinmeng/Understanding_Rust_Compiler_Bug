rust
/// Differentiates between real files and common virtual files. The `Fingerprint` associated with 
/// virtual files is a hash of their source code. It is used to tell them apart and allow incr. comp. 
/// properly match them up in between compilation sessions.
 #[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, RustcDecodable, RustcEncodable)] 
 pub enum FileName { 
     Real(PathBuf), 
     /// A macro.  This includes the full name of the macro, so that there are no clashes. 
     Macros(String), 
     /// call to `quote!` 
     QuoteExpansion(Fingerprint), 
     /// Command line 
     Anon(Fingerprint), 
     /// Hack in src/libsyntax/parse.rs 
     /// FIXME(jseyfried) 
     MacroExpansion(Fingerprint), 
     ProcMacroSourceCode(Fingerprint), 
     /// Strings provided as --cfg [cfgspec] stored in a crate_cfg 
     CfgSpec(Fingerprint), 
     /// Strings provided as crate attributes in the CLI 
     CliCrateAttr(Fingerprint), 
     /// Custom sources for explicit parser calls from plugins and drivers 
     Custom(String), 
} 
