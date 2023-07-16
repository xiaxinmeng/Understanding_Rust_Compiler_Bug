rust
trait AsWorkspaceController<'a> {
    type Controller: for<'b> WorkspaceController<'b> + 'a;

    fn get_controller(&'a mut self) -> Self::Controller;
}
