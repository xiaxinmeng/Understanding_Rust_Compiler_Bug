rust
match xxx {
    NON_STRUCTURAL_MATCH_CONST => {} // Not OK, constant
    NonStructuralMatch { field } => {} // OK, variant
    FLOAT_CONST => {} // Not OK, constant
    1.0 => {} // OK, variant
}
