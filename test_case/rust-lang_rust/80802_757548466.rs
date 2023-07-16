
src/librustdoc/clean/mod.rs
876:    let macro_kind = item.attrs.iter().find_map(|a| {
899:            for mi in item.attrs.lists(sym::proc_macro_derive) {
2040:                    let attrs = item.attrs.clean(cx);

src/librustdoc/clean/inline.rs
437:    item.attrs = merge_attrs(cx, parent_module.into(), load_attrs(cx, did), attrs);
438:    debug!("merged_attrs={:?}", item.attrs);

src/librustdoc/visit_ast.rs
260:                    let please_inline = item.attrs.iter().any(|item| match item.meta_item_list() {
283:                    attrs: &item.attrs,

src/librustdoc/formats/cache.rs
325:                        for alias in item.attrs.get_doc_aliases() {

src/librustdoc/html/render/cache.rs
86:            for alias in item.attrs.get_doc_aliases() {

src/librustdoc/passes/check_code_block_syntax.rs
52:                super::source_span_for_markdown_range(self.cx, &dox, &code_block.range, &item.attrs)
86:                    super::span_of_attrs(&item.attrs).unwrap_or(item.source.span()),
109:        if let Some(dox) = &item.attrs.collapsed_doc_value() {
110:            let sp = span_of_attrs(&item.attrs).unwrap_or(item.source.span());

src/librustdoc/passes/doc_test_lints.rs
40:        let dox = item.attrs.collapsed_doc_value().unwrap_or_else(String::new);
99:            let sp = span_of_attrs(&item.attrs).unwrap_or(item.source.span());
112:            span_of_attrs(&item.attrs).unwrap_or(item.source.span()),

src/librustdoc/passes/non_autolinks.rs
74:        let dox = item.attrs.collapsed_doc_value().unwrap_or_default();
77:                let sp = super::source_span_for_markdown_range(cx, &dox, &range, &item.attrs)
78:                    .or_else(|| span_of_attrs(&item.attrs))

src/librustdoc/doctest.rs
137:                    &krate.item.attrs,
1035:        self.visit_testable(name, &item.attrs, item.hir_id, item.span, |this| {
1041:        self.visit_testable(item.ident.to_string(), &item.attrs, item.hir_id, item.span, |this| {
1047:        self.visit_testable(item.ident.to_string(), &item.attrs, item.hir_id, item.span, |this| {
1053:        self.visit_testable(item.ident.to_string(), &item.attrs, item.hir_id, item.span, |this| {

src/librustdoc/passes/html_tags.rs
184:        let dox = item.attrs.collapsed_doc_value().unwrap_or_default();
188:                let sp = match super::source_span_for_markdown_range(cx, &dox, range, &item.attrs) {
190:                    None => span_of_attrs(&item.attrs).unwrap_or(item.source.span()),

src/librustdoc/passes/propagate_doc_cfg.rs
27:        let new_cfg = match (self.parent_cfg.take(), item.attrs.cfg.take()) {
37:        item.attrs.cfg = new_cfg;

src/librustdoc/html/render/mod.rs
2258:    let cfg = match (&item.attrs.cfg, parent.attrs.cfg.as_ref()) {
2263:    debug!("Portability {:?} - {:?} = {:?}", item.attrs.cfg, parent.attrs.cfg, cfg);
2272:    let cfg = match (&item.attrs.cfg, parent.and_then(|p| p.attrs.cfg.as_ref())) {
2279:        item.attrs.cfg,

src/librustdoc/passes/collect_intra_doc_links.rs
893:        if item.is_mod() && item.attrs.inner_docs {
901:        for (parent_module, doc) in item.attrs.collapsed_doc_value_by_module_level() {
914:                    item.attrs.links.push(link);
920:            if !item.attrs.inner_docs {
1002:        let base_node = if item.is_mod() && item.attrs.inner_docs {
1656:    let attrs = &item.attrs;
