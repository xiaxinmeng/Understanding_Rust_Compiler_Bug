"#;
const CODE_INLINE: &str = r#"`((?s).*?)`"#;
const HEADING1: &str = r#"^#{1}\s+(.*)"#;
const HEADING2: &str = r#"^#{2}\s+(.*)"#;
const HEADING3: &str = r#"^#{3}\s+(.*)"#;
const HEADING4: &str = r#"^#{4}\s+(.*)"#;
const BOLD: &str = r#"\*\*((?s).+?)\*\*"#;
const ITALIC: &str = r#"_((?s).+?)_"#;
const LIST: &str = r#"^[-*]\s(.*?)$"#;
