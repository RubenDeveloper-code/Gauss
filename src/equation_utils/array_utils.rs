mod array_utils {

    pub fn index_of_zeros_in_diagonal(array: Vec<Vec<Term>>) -> Vec<usize> {
        let mut diagonal: usize = 0;
        let mut diagonal_zeros: Vec<usize> = Default::default();
        loop {
            if array[diagonal][diagonal].constant == 0.0 {
                diagonal_zeros.push(diagonal);
            }
            diagonal += 1;
            if diagonal == array.len() {
                break;
            }
        }
        return diagonal_zeros;
    }
}
