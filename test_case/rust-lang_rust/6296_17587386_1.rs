
pub trait ApproxEq<Eps> {
    fn approx_epsilon() -> Eps;
    fn approx_eq(&self, other: &Self) -> bool;
    fn approx_eq_eps(&self, other: &Self, approx_epsilon: &Eps) -> bool;
+   fn relative_epsilon() -> Eps;
+   fn relative_eq(&self, other: &Self) -> bool;
+   fn relative_eq_eps(&self, other: &Self, approx_epsilon: &Eps) -> bool;
}

impl ApproxEq<f32> for f32 {
     #[inline(always)]
     fn approx_epsilon() -> f32 { 1.0e-6 }

     #[inline(always)]
     fn approx_eq(&self, other: &f32) -> bool {
         self.approx_eq_eps(other, &ApproxEq::approx_epsilon::<f32, f32>())
     }

     #[inline(always)]
     fn approx_eq_eps(&self, other: &f32, approx_epsilon: &f32) -> bool {
         (*self - *other).abs() < *approx_epsilon
     }

+   #[inline(always)]
+   fn relative_epsilon() -> f32 { 1.0e-6 }
+
+   #[inline(always)]
+   fn relative_eq(&self, other: &f32) -> bool {
+       self.relative_eq_eps(other, &ApproxEq::relative_epsilon::<f32, f32>())
+   }
+
+   #[inline(always)]
+   fn relative_eq_eps(&self, other: &f32, relative_epsilon: &f32) -> bool {
+       (*self - *other).abs() < fmax(self.abs(), other.abs()) * (*relative_epsilon)
+   }
}
