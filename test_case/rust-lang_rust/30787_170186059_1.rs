
private-in-public-warn.rs:220:20: 220:45 warning: private trait in public interface (error E0445), #[warn(private_in_public)] on by default
private-in-public-warn.rs:220     pub trait Tr2: PrivUseAliasTr<PrivAlias> {} //~ WARN private trait in public interface
                                                 ^~~~~~~~~~~~~~~~~~~~~~~~~
private-in-public-warn.rs:220:20: 220:45 warning: this was fixed in Rust 1.7; it will become a HARD ERROR in a future release!
private-in-public-warn.rs:220     pub trait Tr2: PrivUseAliasTr<PrivAlias> {} //~ WARN private trait in public interface
                                                 ^~~~~~~~~~~~~~~~~~~~~~~~~
private-in-public-warn.rs:220:20: 220:45 note: for more information, see the explanation for E0446 (`--explain E0446`)
private-in-public-warn.rs:220     pub trait Tr2: PrivUseAliasTr<PrivAlias> {} //~ WARN private trait in public interface
                                                 ^~~~~~~~~~~~~~~~~~~~~~~~~
