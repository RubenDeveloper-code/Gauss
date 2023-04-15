pub mod array_utils {
    use super::super::misc_utils::*;
    use super::super::system::*;
    use std::collections::HashSet;

    pub fn print_array(array: Vec<Vec<system::Term>>) {
        let variables = get_variables_in_array(array.clone());
        println!();
        for var in variables.clone() {
            print!(" {}  ", var);
        }
        let line: String = String::from("▀▀▀");
        println!("\n{}", line.repeat(variables.len() + 1));
        for row in array {
            for term in row {
                if term.pos == RIGHT {
                    print!(" █ ");
                }
                print!(" {} ", misc_utils::float2print(term.constant));
            }
            print!("\n");
        }
    }
    pub fn replace_row_in_array<T: Clone>(
        array: Vec<Vec<T>>, new_row: Vec<T>, index_row_to_replace: usize,
    ) -> Vec<Vec<T>> {
        let mut new_array = array.clone();
        new_array.remove(index_row_to_replace);
        new_array.insert(index_row_to_replace, new_row);
        return new_array;
    }
    pub fn flip_rows_in_array<T: Clone>(
        array: Vec<Vec<T>>, first_row: usize, second_row: usize,
    ) -> Vec<Vec<T>> {
        let mut flip_array = array.clone();
        flip_array.remove(first_row);
        flip_array.insert(first_row, array[second_row].clone());
        flip_array.remove(second_row);
        flip_array.insert(second_row, array[first_row].clone());
        return flip_array;
    }

    pub fn index_of_zeros_in_diagonal(array: Vec<Vec<system::Term>>) -> Vec<usize> {
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
    pub fn get_array_dimentions<T: Clone>(array: Vec<Vec<T>>) -> (usize, usize) {
        let m = array.len() - 1;
        let n = array[0].len() - 1;
        return (m, n);
    }
    pub fn get_variables_in_array(array: Vec<Vec<system::Term>>) -> Vec<String> {
        let mut variables: Vec<String> = Default::default();
        for row in array.into_iter() {
            for term in row {
                if !variables.contains(&term.variable) {
                    variables.push(term.variable);
                }
            }
        }
        return variables;
    }
    pub fn get_variables_in_row(row_of_terms: Vec<system::Term>) -> HashSet<String> {
        return row_of_terms
            .clone()
            .into_iter()
            .map(|term| term.variable.clone())
            .collect();
    }
    pub fn get_vector_of_variables_in_row(row_of_terms: Vec<system::Term>) -> Vec<String> {
        return row_of_terms
            .clone()
            .into_iter()
            .map(|term| term.variable.clone())
            .collect();
    }
}
