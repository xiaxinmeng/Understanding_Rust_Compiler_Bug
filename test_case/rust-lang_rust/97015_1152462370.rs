rs
impl<'a> From<&'a mut [u8]> for BorrowBuf<'a> {
impl<'a> From<&'a mut [MaybeUninit<u8>]> for BorrowBuf<'a> {
impl<'a> BorrowBuf<'a> {
    pub fn capacity(&self) -> usize {
    pub fn len(&self) -> usize {
    pub fn init_len(&self) -> usize {
    pub fn filled(&self) -> &[u8] {
    pub fn unfilled<'b>(&'b mut self) -> BorrowCursor<'a, 'b> {
    pub fn clear(&mut self) -> &mut Self {
    pub unsafe fn set_init(&mut self, n: usize) -> &mut Self {
impl<'a, 'b> BorrowCursor<'a, 'b> {
    pub fn clone<'c>(&'c mut self) -> BorrowCursor<'a, 'c> {
    pub fn capacity(&self) -> usize {
    pub fn written(&self) -> usize {
    pub fn init_ref(&self) -> &[u8] {
    pub fn init_mut(&mut self) -> &mut [u8] {
    pub fn uninit_mut(&mut self) -> &mut [MaybeUninit<u8>] {
    pub unsafe fn as_mut(&mut self) -> &mut [MaybeUninit<u8>] {
    pub unsafe fn advance(&mut self, n: usize) -> &mut Self {
    pub fn ensure_init(&mut self) -> &mut Self {
    pub unsafe fn set_init(&mut self, n: usize) -> &mut Self {
    pub fn append(&mut self, buf: &[u8]) {
