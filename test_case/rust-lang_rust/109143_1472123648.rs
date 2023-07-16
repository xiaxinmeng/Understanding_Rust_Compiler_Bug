rust
-                                    LifetimeRibKind::Generics { .. } => None,
-                                    LifetimeRibKind::ConstGeneric | LifetimeRibKind::AnonConst => {
+                                    LifetimeRibKind::Generics { .. } | LifetimeRibKind::ConstGeneric  => None,
+                                    LifetimeRibKind::AnonConst => {
                                         span_bug!(ident.span, "unexpected rib kind: {:?}", rib.kind)
                                     }

