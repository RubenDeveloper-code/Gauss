pub mod exceptions {
    use super::super::array_utils::*;
    use super::super::gauss::*;
    use super::super::misc_utils::*;
    use super::super::system::*;

    pub fn array_is_invalid(array: Vec<Vec<system::Term>>) -> bool {
        let (m, _) = array_utils::get_array_dimentions(array.clone());
        let mut variables: Vec<String> = array_utils::get_variables_in_array(array.clone());
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
            let test_variables = array_utils::get_vector_of_variables_in_row(row.clone());
            if test_variables.len() != variables.len() {
                println!("ERROR: variables duplicadas o constantes sin sumar");
                return true;
            }
        }
        if m > variables.len() - 2 {
            println!("ERROR: mas ecuaciones de las necesarias");
            return true;
        }

        return false;
    }
    pub fn results_are_invalid(unknowns: Vec<gauss::Unknown>) -> bool {
        if unknowns.into_iter().any(|term| term.constant.is_nan()) {
            println!("Resultados invalidos | infinitas soluciones");
            return true;
        }
        return false;
    }
}
