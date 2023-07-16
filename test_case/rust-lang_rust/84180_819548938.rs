rust
pub struct Foo(u32);

impl Foo {
    pub fn foo(&self) -> Result<&Foo, ()> {
        Ok(self)
    }

    pub fn bar(&self) -> Result<u32, ()> {
        Ok(self.0)
    }
}

pub fn bar(f: &Foo) -> Result<u32, ()> {
    let x = f
    	.foo()?
    	.bar()?;

    Ok(x + 3)
}

#[cfg(test)]
mod tests {
    use super::{bar, Foo};

    #[test]
    fn test_it() {
        let f = Foo(5);
        let result = bar(&f);
        assert_eq!(result, Ok(8))
    }
}
