
/* Normal integer and rationals algebras */
pub trait AdditiveMonoid : Zero + Add<Self,Self> {}
pub trait AdditiveAbelianGroup : AdditiveMonoid + Sub<Self,Self> + Neg<Self> {}
pub trait CRing : AdditiveAbelianGroup + Mul<Self,Self> + One {}
pub trait Field : CRing + Div<Self,Self> {}

impl<F: Zero + Add<F, F>> AdditiveMonoid for F {}
impl<F: AdditiveMonoid + Sub<F, F> + Neg<F>> AdditiveAbelianGroup for F {}
impl<F: AdditiveAbelianGroup + Mul<F, F> + One> CRing for F {}
impl<F: CRing + Div<F, F>> Field for F {}

pub trait ScalarMultiplication<F> : Mul<F, Self> + Div<F, Self> {}
impl<F, V: Mul<F, V> + Div<F, V>> ScalarMultiplication<F> for V {}

/* A Point is an element of a Vector Space over scalar field F */
pub trait Point<F: Field> : Eq + AdditiveAbelianGroup + ScalarMultiplication<F> {}

/*
 * AdditiveMonoid: You can .sum() this, with Ord these are non-negative edge weigths,
 * AdditiveAbelianGroup + Ord: You can .abs() this, general graph edge weights
 * Field: Simplest possible general Scalar/float/Num
 * 
 *  Point: is a vector space element so it applies to any usual scalar, graphical point, matrix.
 *  needs more traits for dot product, norm, distance, cross product, etc.
 */
