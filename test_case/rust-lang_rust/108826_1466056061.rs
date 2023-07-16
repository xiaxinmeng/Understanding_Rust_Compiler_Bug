diff
- fn next<'a>(&'a mut self) -> Self::Item<'a>
+ fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> 
