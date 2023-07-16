rust
pub struct Expr {
    pub super_class: Option<Box<Expr>>,
}
#[allow(dead_code, non_upper_case_globals)]
const DERIVE_FOLD_FOR_Expr: () =
    {
        extern crate swc_common;
        impl <__Folder> swc_common::FoldWith<__Folder> for Expr {
            fn fold_children(self, __folder: &mut __Folder) -> Self {
                match self {
                    Expr { super_class: _super_class } => {
                        return Expr{super_class:
                                        swc_common::Folder::<Option<Box<Expr>>>::fold(__folder,
                                                                                      _super_class),};
                    }
                }
            }
        }
    };

