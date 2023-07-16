
pub trait FullMultiplication {
    type Multiplicand;
    type MulResult;
}

impl<const N: usize> FullMultiplication for BigintRepresentation<{N}> 
{
    type Multiplicand = BigintRepresentation<{N}>;
    type MulResult = BigintRepresentation<{N*2}>;
}
