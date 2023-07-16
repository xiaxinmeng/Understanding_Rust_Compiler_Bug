rust
/// Each Vietnamese monosyllabic word can be decomposed at most into 4 parts,
/// which are:
///
/// 1. The initial group of consonants
/// 1. The vowel group
/// 1. The group of final consonants
/// 1. The tonal accent
///
/// # Examples
///
/// Word            | Consonant | Vowel | Consonant | Accent
/// --------------- | --------- | ----- | --------- | ------
/// trường (school) | tr        | ươ    | ng        | Grave
/// muối (salt)     | m         | uôi   |           | Acute
/// ao (the pond)   |           | ao    |           |
