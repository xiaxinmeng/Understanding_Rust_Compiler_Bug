rust
trait ID {}
struct Object<T: ID> {}
impl<T: ID> Object<T: ID> { /* shared methods */ }

struct FileID {}
impl ID for FileID {}
pub type File = Object<FileID>;
impl File { /* file-specific methods */ }

struct GroupID {}
impl ID for GroupID {}
pub type Group = Object<GroupID>;
impl Group { /* group-specific methods */ }

trait ContainerID : ID {}
impl ContainerID for FileID {}
impl ContainerID for GroupID {}
impl<T: ContainerID> Object<T> { /* container methods */ }
