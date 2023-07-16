
Iterator::intersperse_with<G>(self, separator_gen: G) -> IntersperseWith<Self, G> where Self:Sized, G: FnMut() -> Self::Item
Iterator::intersperse(self, separator: Self::Item) -> Intersperse<Self> where Self:Sized, Self::Item: Clone
