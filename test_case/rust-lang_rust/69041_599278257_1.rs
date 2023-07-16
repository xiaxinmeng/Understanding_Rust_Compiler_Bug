diff
- quote!(try!(_serde::de::SeqAccess::next_element::<#field_ty>(&mut __seq)))
+ let span = Span::def_site().located_at(field.original.span());
+ let func = quote_spanned!(span=> _serde::de::SeqAccess::next_element::<#field_ty>);
+ quote!(try!(#func(&mut __seq)))
