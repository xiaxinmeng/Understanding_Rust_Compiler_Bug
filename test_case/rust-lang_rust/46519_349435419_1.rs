rust
#[derive(Clone, Debug, Eq, MallocSizeOf, PartialEq, ToCss)]
/// Allows authors to explicitly specify the language system of the font,
/// overriding the language system implied by the content language
pub enum FontLanguageOverride {
    /// When rendering with OpenType fonts,
    /// the content language of the element is
    /// used to infer the OpenType language system
    Normal,
    /// Single three-letter case-sensitive OpenType language system tag,
    /// specifies the OpenType language system to be used instead of
    /// the language system implied by the language of the element
    Override(Box<str>),
    /// Use system font
    System(SystemFont)
}

impl FontLanguageOverride {
    /// Get `font-language-override` with `system font`
    pub fn system_font(f: SystemFont) -> Self {
        FontLanguageOverride::System(f)
    }
}

        <%
            system_fonts = """caption icon menu message-box small-caption status-bar
                              -moz-window -moz-document -moz-workspace -moz-desktop
                              -moz-info -moz-dialog -moz-button -moz-pull-down-menu
                              -moz-list -moz-field""".split()
        %>

        #[derive(Clone, Copy, Debug, Eq, Hash, MallocSizeOf, PartialEq, ToCss)]
        pub enum SystemFont {
            % for font in system_fonts:
                ${to_camel_case(font)},
            % endfor
        }
