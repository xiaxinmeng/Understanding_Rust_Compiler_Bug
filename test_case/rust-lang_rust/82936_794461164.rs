rust
enum ConstantSource {
    Param(ParamConst),
    Bound(...),
    Unevaluated(...),
    Value(...),
    Err(...),
}
