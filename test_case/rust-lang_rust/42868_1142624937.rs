
fn lister<'a, 'b, T: LifetimedDisplay<'a>>(
    buffer: &'b mut dyn Write,
    element: project::element::ElementRef,
    project: &'a project::project::Project,
) {
    writeln!(buffer, "{}", T::new(element, project)).unwrap();
}
