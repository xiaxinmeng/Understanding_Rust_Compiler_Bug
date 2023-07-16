
trait GenericBackend {
    type ParentObject: GenericParentObject<Self>;
    type ChildObject: GenericChildObject<Self>;
    fn parent() -> Self::ParentObject;
}
trait GenericParentObject<Backend: GenericBackend> {
    fn get_children(&self) -> Vec<Backend::ChildObject>;
}
trait GenericChildObject<Backend: GenericBackend> {
}
