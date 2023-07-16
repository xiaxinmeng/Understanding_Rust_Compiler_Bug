
warning: unnecessary braces around block return value
  --> strategies/src/types.rs:67:34
   |
67 |     fn asset_type(&self) -> &str { self.asset_type.as_ref() }
   |                                  ^^                        ^^
   |
help: remove these braces
   |
67 -     fn asset_type(&self) -> &str { self.asset_type.as_ref() }
67 +     fn asset_type(&self) -> &str self.asset_type.as_ref()
