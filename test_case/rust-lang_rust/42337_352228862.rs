
            quote! {
                extern crate std;
                extern crate postgres;
                use std::io::Write;
                use postgres::types::{IsNull, ToSql, Type};

                impl ToSql for #table_ident {
                    fn to_sql<W: Write + ?Sized>(&self, ty: &Type, out: &mut W) -> postgres::Result<IsNull> {
                        self.#primary_key_ident.to_sql(ty, out)
                    }

                    fn accepts(ty: &Type) -> bool {
                        match *ty {
                            Type::Int4 => true,
                            _ => false,
                        }
                    }

                    fn to_sql_checked(&self, ty: &Type, out: &mut Write) -> postgres::Result<IsNull> {
                        if !<Self as ToSql>::accepts(ty) {
                            return Err(postgres::error::Error::WrongType(ty.clone()));
                        }
                        self.to_sql(ty, out)
                    }
                }
            }
