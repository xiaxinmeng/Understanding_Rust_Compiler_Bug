
    fn build_arm_parse(&self, index: usize, kind: ArgKind) -> TokenStream {
        let temp_ident = &self.info.temp_ident;
        let msg = format!("parameter `{}` speficied more than once", self.name);
        let span = self.info.field.span();
        let expr = self.ty.build_parse_expr(kind, span);
        let var = kind.to_helper_name_index_variant();
        quote_spanned! { span=>
            ::structmeta::helpers::NameIndex::#var(Ok(#index)) => {
                if #temp_ident.is_some() {
                    return Err(::syn::Error::new(span, #msg));
                }
                #temp_ident = Some(#expr);
            }
        }
    }
