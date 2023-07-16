rust
impl From<Zsh> for Cmd {
    fn from(value: Zsh) -> Self { Self::Zsh }
}
impl std::convert::TryFrom<Cmd> for Zsh {
    type Error = Cmd;
    fn try_from(value: Cmd) -> Result<Self, Self::Error> { if let Cmd::Zsh = value { Ok(Zsh) } else { Err(value) } }
}
#[doc = " a zsh script to execute"]
struct Zsh;
impl From<Bash> for Cmd {
    fn from(value: Bash) -> Self { Self::Bash }
}
impl std::convert::TryFrom<Cmd> for Bash {
    type Error = Cmd;
    fn try_from(value: Cmd) -> Result<Self, Self::Error> { if let Cmd::Bash = value { Ok(Bash) } else { Err(value) } }
}
struct Bash;
impl Cmd {
    fn cast<U: ?Sized>(self) -> Box<U> where Zsh: ::core::marker::Unsize<U>, Bash: ::core::marker::Unsize<U> {
        let value = self;
        let value = match Zsh::try_from(value) {
            Ok(v) => {
                let x = Box::new(v);
                return x;
            }
            Err(v) => v,
        };
        let value = match Bash::try_from(value) {
            Ok(v) => {
                let x = Box::new(v);
                return x;
            }
            Err(v) => v,
        };
        unreachable!();
    }
}
