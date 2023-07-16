
stringy_enum! {
    pub enum SectionKind (parse error TreeError::ParseSectionKind) {
        Preface = "preface",
        Chapter = "chapter",
        Sect1 = "sect1",
        Sect2 = "sect2",
    }
}
