rust
error[E0308]: mismatched types
   --> src\libpanic_unwind\seh64_gnu.rs:131:9
    |
131 |         EHAction::None => None,
    |         ^^^^^^^^^^^^^^ expected enum `core::result::Result`, found enum `dwarf::eh::EHAction`
    |
    = note: expected type `core::result::Result<dwarf::eh::EHAction, ()>`
               found type `dwarf::eh::EHAction`
