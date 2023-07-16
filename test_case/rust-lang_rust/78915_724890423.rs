
$ rg "BorrowckMode::Mir|\.migrate|suppress_errors" compiler
compiler/rustc_middle/src/ty/context.rs
1336:        self.borrowck_mode().migrate()
1365:            return BorrowckMode::Mir;

compiler/rustc_infer/src/infer/lexical_region_resolve/mod.rs
45:        RegionckMode::Erase { suppress_errors: false } => {
56:        RegionckMode::Erase { suppress_errors: true } => {

compiler/rustc_infer/src/infer/mod.rs
105:        suppress_errors: bool,
124:            BorrowckMode::Migrate => RegionckMode::Erase { suppress_errors: false },
127:            BorrowckMode::Mir => RegionckMode::Erase { suppress_errors: true },

compiler/rustc_mir/src/borrow_check/mod.rs
1118:                ) if { tcx.migrate_borrowck() && this.borrow_set.contains(&location) } => {

compiler/rustc_data_structures/src/sso/map.rs
528:        self.ssomap.migrate_if_full();

compiler/rustc_session/src/config.rs
483:            BorrowckMode::Mir => false,
1568:        "mir" => BorrowckMode::Mir,
