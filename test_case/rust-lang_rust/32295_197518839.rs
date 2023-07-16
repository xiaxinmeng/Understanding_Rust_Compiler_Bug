 rust
trait Mat<D: NdFloat> {
    fn submatrix(self, rows: &Vec<Ix>, columns: &Vec<Ix>) -> Self;
}

impl<D: NdFloat> Mat<D> for MA {
    fn submatrix(self, rows: &Vec<Ix>, columns: &Vec<Ix>) -> MA {
        let mut x = MA::zeros((rows.len(), columns.len()));
        for ((i,&io),(j,&jo)) in rows.iter().enumerate().zip(columns.iter().enumerate()) {
            x[(i,j)] = self[(io,jo)];
        }
        x
    }
}
