
impl<S> Map<S>
    where S: HashState,
          <S as HashState>::Wut: Hasher<Output=u64>,
