
unsafe fn foo<'a>(src: &mut ManuallyDrop<&'static ()>) -> &'a () {
    ManuallyDrop::<&'a ()>::take(src)
}
