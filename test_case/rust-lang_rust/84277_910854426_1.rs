rust
pub fn viewport<'p>(
    viewports: &'p HashMap<ViewportId, Viewport>,
    viewport_id: &ViewportId,
) -> Result<&'p Viewport, BindingError> {
    viewports
        .get(viewport_id)
        .ok_or(BindingError::ViewportDoesNotExist)
}
pub fn viewport_mut<'p, 'msg>(
    viewports: &'p mut HashMap<ViewportId, Viewport>,
    viewport_id: &ViewportId,
) -> Result<&'p mut Viewport, BindingError> {
    viewports
        .get_mut(viewport_id)
        .ok_or(BindingError::ViewportDoesNotExist)
}
...
