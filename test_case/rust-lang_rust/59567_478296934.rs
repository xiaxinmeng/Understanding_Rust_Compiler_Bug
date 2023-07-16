
pub trait Close {
    fn close(self) -> Result<(), std::io::Error>;
}
