rust
// Could even make this not constructible without calling
// a method that registers something similar to delay_bug.
// But that's overkill for now.
pub struct AlwaysError;

mod sealed {
	use super::{AlwaysError, ErrorReported, Level};

    pub trait LevelTrait {
        type Emitted;
        fn emit(self) -> (Level, Self::Emitted);
    }

    impl LevelTrait for Level {
        type Emitted = ();
        fn emit(self) -> (Level, Self::Emitted) {
            (self, ())
        }
    }

    impl LevelTrait for AlwaysError {
        type Emitted = ErrorReported;
        fn emit(self) -> (Level, Self::Emitted) {
            (Level::Error, ErrorReported { private: () })
        }
    }
}

struct DiagnosticBuilder<L> {
    level: L,
    ...
}

impl<L: sealed::LevelTrait> DiagnosticBuilder<L> {
    pub fn emit(self) -> L::Emitted {
        ...
        let (level, emitted) = self.level.emit();
        ...
        emitted
    }
}
