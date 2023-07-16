
pub enum DefinedAttr {
    AttrCrateType(bool), // is_lib
    AttrFeature(~[AttrFeatureItem]), // enum AttrFeatureItem { Globs, MacroRules, ... }
    AttrNoUv, // no arg
    ...
}
