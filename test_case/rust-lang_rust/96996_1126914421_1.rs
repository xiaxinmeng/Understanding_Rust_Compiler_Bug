diff
- type StrRef<'a> = impl AsRef<str>;
+ type StrRef<'a> = impl AsRef<str> + 'a;
