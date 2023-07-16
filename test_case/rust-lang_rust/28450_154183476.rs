
struct Private;
pub use Private as Public; // Makes `Private` exported, now it can be used in external interfaces
pub type Public = Private; // Makes `Private` exported, now it can be used in external interfaces
