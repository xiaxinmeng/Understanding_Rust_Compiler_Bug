rust
struct Definitions {
    // Both before and after (or filled before, used after)
    table: DefPathTable,
    parent_modules_of_macro_defs: FxHashMap<ExpnId, DefId>,
    expansions_that_defined: FxHashMap<LocalDefId, ExpnId>,

    // Before
    def_id_to_span: IndexVec<LocalDefId, Span>,
    node_id_to_def_id: FxHashMap<ast::NodeId, LocalDefId>,
    def_id_to_node_id: IndexVec<LocalDefId, ast::NodeId>,
    next_disambiguator: FxHashMap<(LocalDefId, DefPathData), u32>,
    invocation_parents: FxHashMap<ExpnId, LocalDefId>,
    placeholder_field_indices: FxHashMap<ast::NodeId, usize>,

    // ???
    node_id_to_hir_id: IndexVec<ast::NodeId, Option<hir::HirId>>,
    hir_id_to_node_id: FxHashMap<hir::HirId, ast::NodeId>,
}
