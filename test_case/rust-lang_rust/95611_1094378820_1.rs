rust
#[derive(Data, Clone, PartialEq, Serialize, Deserialize, Debug)]
struct Row {
	a: String,
	b: u64,
	c: f64,
	// d: List<Row>,
	e: List<Value>,
}

#[inline(always)]
fn hash_a<H>(&self, _state: &mut H)
where
    H: __::Hasher,
    for<'a> __::Wrapper<'a, Row>: __::Hash,
{ ... }
