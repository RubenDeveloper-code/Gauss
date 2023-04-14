pub mod exceptions {
    use super::super::system::system::*;
    use super::super::system::*;

    pub fn array_is_invalid(array: Vec<Vec<Term>>) -> bool {
        let (n, m) = equation_utilities::get_array_dimentions(array.clone());
        let mut variables: Vec<String> = equation_utilities::get_variables_in_array(array.clone());
        variables.sort();
        if m == 0 {
            println!("ERROR: Solo se encontro una ecuacion");
            return true;
        }
        if variables.is_empty() {
            println!("ERROR: No se encontraron variables a encontrar");
            return true;
        }
        if variables.len() == 1 {
            println!("ERROR: creo que ya casi tienes el resultado...");
            return true;
        }
        for (it, row) in array.iter().enumerate() {
            if !row.into_iter().any(|term| term.pos == RIGHT) {
                println!("ERROR: No hay nada que encontrar en la fila {}", it);
                return true;
            }
            let test_variables = equation_utilities::get_vector_of_variables_in_row(row.clone());
            if test_variables.len() != variables.len() {
                println!("ERROR: variables duplicadas");
                return true;
            }
        }

        return false;
    }
}
