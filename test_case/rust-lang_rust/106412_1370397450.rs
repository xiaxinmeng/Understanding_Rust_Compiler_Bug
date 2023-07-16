diff
-      ItemId::Primitive(ty, krate) => Id(format!("p:{}:{}", krate.as_u32(), ty.as_sym())), 
+      ItemId::Primitive(_, _) => unreachable!(),
