
                    (_, &mir::CallKind::ConvergingCleanup { targets: (target, _), .. }) |
                    (_, &mir::CallKind::Converging { target, .. }) => {
