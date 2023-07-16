
class Ring self
class Field self
class VectorSpace a self
class (Ring self, Field s, VectorSpace s rv, VectorSpace s cv, Matrix s cv rv self mt) => Matrix s rv cv mt self where
    transpose :: self -> mt
