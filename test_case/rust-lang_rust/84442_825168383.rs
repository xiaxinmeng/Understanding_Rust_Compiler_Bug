rust
                        match &meta_item.kind {
                            MetaItemKind::List(..) => {
                                error!(r#"expected `key` or `key="value"`"#);
                            }
                            MetaItemKind::NameValue(lit) if !lit.kind.is_str() => {
                                error!("argument value must be a string");
                            }
                            MetaItemKind::NameValue(..) | MetaItemKind::Word => {
                                let ident = meta_item.ident().expect("multi-segment cfg key");
                                return (ident.name, meta_item.value_str());
                            }
                        }
