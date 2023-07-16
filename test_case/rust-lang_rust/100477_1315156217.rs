
impl<'a> Debug for Document<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        #[derive(Debug)]
        struct Doc<'a> {
            elements: &'a Vec<DocumentElement<'a>>,
        }
        Doc { elements: &self.elements }.fmt(f)
    }
}
