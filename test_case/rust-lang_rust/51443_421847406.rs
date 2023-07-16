
trait SelfOps{
    fn piped_ref<'a,F, U>(&'a self, f: F) -> U
    where
        F: FnOnce(&'a Self) -> U,
    {
        f(self)
    }

    fn format_debug(&self)->String
    where Self:fmt::Debug
    {
        format!("{:?}",self)
    }
}

impl<This:?Sized> SelfOps for This{}

