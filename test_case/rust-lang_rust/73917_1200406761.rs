rust
struct LinkGuard {
    deps: &'static [&'static LinkGuard],
}

extern "Rust" {
    // foo[b7924b40b7ed5e7f]::{shim:LINK_GUARD#0}::<0x461e83de35f0b704f7e69b4cc741ad8eu128>
    #[link_name = "_RINSCsfL95rG4I7iB_3foo10LINK_GUARDKo461e83de35f0b704f7e69b4cc741ad8e_E"] 
    static LINK_GUARD_DEP_FOO: LinkGuard;

    // bar[acb4b2d152c0bd2e]::{shim:LINK_GUARD#0}::<0xf8fc0fadc6a6e727eef4b916531abfe9u128>
    #[link_name = "_RINSCsePjaApBJGQA_3bar10LINK_GUARDKof8fc0fadc6a6e727eef4b916531abfe9_E"] 
    static LINK_GUARD_DEP_BAR: LinkGuard;
}

// my_crate[78009e3fbfa2f6af]::{shim:LINK_GUARD#0}::<0xe538955c5950b59a598304a1e701c9fbu128>
#[export_name = "_RINSCsaiLK1vfX74x_8my_crate10LINK_GUARDKoe538955c5950b59a598304a1e701c9fb_E"]
pub static LINK_GUARD: LinkGuard {
    deps: unsafe { &[
        &LINK_GUARD_DEP_FOO,
        &LINK_GUARD_DEP_BAR,
    ] }
};
