diff
pub struct ElementUpdater<'a, C>(&'a C);

pub struct AttributesOnly<'er, C>(ElementUpdater<'er, C>);

impl<'er, C> HtmlElementUpdaterMut<C> for AttributesOnly<'er, C> {
    fn html_element_updater_mut<'a>(&'a mut self) -> &'er mut ElementUpdater<'a, C> {
+       assert_outlives::<'a, 'er>(); // <~ :(
        &mut self.0
    }
}

pub trait HtmlElementUpdaterMut<C> {
    fn html_element_updater_mut<'a>(&'a mut self) -> &'a mut ElementUpdater<'a, C>;
}

+ fn assert_outlives<'a: 'b, 'b>() {}
