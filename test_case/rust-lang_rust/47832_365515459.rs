rust
impl Index<PlacementId> for [ScoredMove] {
    type Output = ScoredMove;

	#[inline]
    fn index(&self, index: PlacementId) -> &Self::Output {
		self.index(index.0 as usize)
    }
}

impl Index<PlacementId> for Vec<ScoredMove> {
    type Output = ScoredMove;

	#[inline]
    fn index(&self, index: PlacementId) -> &Self::Output {
		self.index(index.0 as usize)
    }
}
