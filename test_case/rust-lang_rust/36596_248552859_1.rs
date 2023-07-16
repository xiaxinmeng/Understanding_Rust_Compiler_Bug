 rust
    fn as_failure_str(&self) -> &'static str {
        match self {
            &TypeOrigin::Misc(_) |
            &TypeOrigin::RelateOutputImplTypes(_) |
            &TypeOrigin::ExprAssignable(_) => "mismatched types",
            &TypeOrigin::MethodCompatCheck(_) => "method not compatible with trait",
            &TypeOrigin::MatchExpressionArm(.., source) => match source {
                hir::MatchSource::IfLetDesugar{..} => "`if let` arms have incompatible types",
                _ => "match arms have incompatible types",
            },
            &TypeOrigin::IfExpression(_) => "if and else have incompatible types",
            &TypeOrigin::IfExpressionWithNoElse(_) => "if may be missing an else clause",
            &TypeOrigin::RangeExpression(_) => "start and end of range have incompatible types",
            &TypeOrigin::EquatePredicate(_) => "equality predicate not satisfied",
            &TypeOrigin::MainFunctionType(_) => "main function has wrong type",
            &TypeOrigin::StartFunctionType(_) => "start function has wrong type",
            &TypeOrigin::IntrinsicType(_) => "intrinsic has wrong type",
            &TypeOrigin::MethodReceiver(_) => "mismatched method receiver",
        }
    }
