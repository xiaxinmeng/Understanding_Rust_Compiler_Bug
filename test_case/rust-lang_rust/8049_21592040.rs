
/* Element of a Vector Space over scalar field F: 
 * Additive Abelian Group and multiplication with scalar */
pub trait Point<F> : Eq +
             Zero + Add<Self,Self> + Sub<Self,Self> + Neg<Self> +
             Mul<F,Self> + Div<F,Self> {}
