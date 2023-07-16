
trait DataType {
   type DiskType;
   const DISK_SIZE: usize;
   // ...
}

struct UInt32;

impl DataType {
   const DiskType = u32;
   const DISK_SIZE: usize = std::mem::size_of<Self::DiskType>();
}
