diff
pub fn request_ref<'a, T>(provider: &'a (impl Provider + ?Sized)) -> Option<&'a T>
where
    T: 'static + ?Sized,
{
    request_by_type_tag::<'a, tags::Ref<tags::MaybeSizedValue<T>>>(provider)
}

+pub fn request_ref_mut<'a, T>(provider: &'a (impl Provider + ?Sized)) -> Option<&'a mut T>
+where
+    T: 'static + ?Sized,
+{
+    request_by_type_tag::<'a, tags::RefMut<tags::MaybeSizedValue<T>>>(provider)
+}

mod tags {
     pub struct Ref<I>(PhantomData<I>);
 
     impl<'a, I: MaybeSizedType<'a>> Type<'a> for Ref<I> {
         type Reified = &'a I::Reified;
     }
+    
+    pub struct RefMut<I>(PhantomData<I>);
+
+    impl<'a, I: MaybeSizedType<'a>> Type<'a> for RefMut<I> {
+        type Reified = &'a mut I::Reified;
+    }
}
