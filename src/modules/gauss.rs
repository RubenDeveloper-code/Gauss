pub mod gauss {
    use super::super::array_utils::*;
    use super::super::exceptions::*;
    use super::super::misc_utils::*;
    use super::super::system::*;

    #[derive(Debug, Clone)]
    pub struct Unknown {
        pub constant: f64,
        pub variable: String,
    }

    pub fn unknowns_from_system(array: Vec<Vec<system::Term>>) -> Option<Vec<Unknown>> {
        if exceptions::array_is_invalid(array.clone()) {
            return None;
        }
        let results = solve(array.clone());
        if exceptions::results_are_invalid(results.clone()) {
            return None;
        }
        return Some(results);
    }

    pub fn print_unknows(unknowns: Vec<Unknown>) {
        let mut unknowns = unknowns;
        unknowns.reverse();
        println!("\n▄▄▄▄▄▄▄▄▄Resultados▄▄▄▄▄▄▄▄▄▄");
        for unknown in unknowns {
            println!("{} = {}", unknown.variable, misc_utils::float2print(unknown.constant));
        }
    }

    fn solve(array: Vec<Vec<system::Term>>) -> Vec<Unknown> {
        let (m, n) = array_utils::get_array_dimentions(array.clone());
        let variables = array_utils::get_variables_in_array(array.clone());
        let mut unknowns: Vec<Unknown> = Default::default();
        if n - 1 != m {
            unknowns = values_for_missing_unknowns(missing_unknowns(variables, m));
        }
        return get_unknowns(apply_gauss_method(array), unknowns.clone());
    }

    fn missing_unknowns(variables: Vec<String>, m: usize) -> Vec<String> {
        let missing_unknowns = variables
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| i > m)
            .map(|(_, missing_unknown)| missing_unknown)
            .collect::<Vec<String>>();
        return missing_unknowns;
    }

    fn values_for_missing_unknowns(missing_unknowns: Vec<String>) -> Vec<Unknown> {
        println!("{:?}", missing_unknowns);
        let mut unknowns: Vec<Unknown> = Default::default();
        for missing_unknown in missing_unknowns {
            if missing_unknown == "=" {
                continue;
            }
            println!("De valor a: {} (default: 1)", missing_unknown);
            let constant = misc_utils::input_float();
            let variable = missing_unknown;
            let unknown = Unknown { constant, variable };
            unknowns.push(unknown);
        }
        return unknowns;
    }

    fn apply_gauss_method(array: Vec<Vec<system::Term>>) -> Vec<Vec<system::Term>> {
        let mut array = array.clone();
        let (_m, _) = array_utils::get_array_dimentions(array.clone());

        let mut diagonal: usize = 0;
        let mut m: usize = _m;

        array = fix_array_if_need(array.clone());

        loop {
            let pivot = get_pivot(array.clone(), diagonal);
            array = array_utils::replace_row_in_array(array.clone(), pivot.clone(), diagonal);
            loop {
                let operated_row = solve_row(array.clone(), pivot.clone(), m, diagonal);
                array = array_utils::replace_row_in_array(array.clone(), operated_row, m);
                array_utils::print_array(array.clone());
                if m - 1 <= diagonal {
                    break;
                } else {
                    m -= 1;
                }
            }
            diagonal += 1;
            m = _m;
            if diagonal == _m {
                break;
            }
        }
        return array;
    }

    fn fix_array_if_need(array: Vec<Vec<system::Term>>) -> Vec<Vec<system::Term>> {
        let mut array = array;
        loop {
            let zeros_in_diagonal = array_utils::index_of_zeros_in_diagonal(array.clone());
            if zeros_in_diagonal.is_empty() {
                break;
            }
            let target: i8 = if zeros_in_diagonal.first().unwrap() + 1 >= array.len() {
                -1
            } else {
                1
            };
            array = array_utils::flip_rows_in_array(
                array.clone(),
                *zeros_in_diagonal.first().unwrap(),
                (*zeros_in_diagonal.first().unwrap() as i8 + target) as usize,
            );
            println!("\n>>Arreglo acomodado");
            array_utils::print_array(array.clone());
        }
        return array;
    }

    fn solve_row(
        array: Vec<Vec<system::Term>>, pivot: Vec<system::Term>, m: usize, diagonal: usize,
    ) -> Vec<system::Term> {
        let multiplier = array[m][diagonal].constant;
        let row: Vec<system::Term> = array[m]
            .clone()
            .into_iter()
            .enumerate()
            .map(|(it, mut term)| {
                term.constant += pivot[it].constant * multiplier * NEGATIVE;
                term
            })
            .collect();
        println!("\n>{}R - {}R{}", m + 1, multiplier, diagonal + 1);
        return row;
    }
    fn get_pivot(array: Vec<Vec<system::Term>>, diagonal: usize) -> Vec<system::Term> {
        let pivot = array[diagonal]
            .clone()
            .into_iter()
            .map(|mut term| {
                term.constant /= array[diagonal][diagonal].constant;
                term
            })
            .collect::<Vec<system::Term>>();
        return pivot;
    }
    fn get_unknowns(mut array: Vec<Vec<system::Term>>, unknowns: Vec<Unknown>) -> Vec<Unknown> {
        let mut unknowns = unknowns;
        array.reverse();
        for row in array.into_iter() {
            unknowns.push(get_unknown(row, unknowns.clone()));
        }
        return unknowns;
    }
    fn get_unknown(row: Vec<system::Term>, unknowns: Vec<Unknown>) -> Unknown {
        let unknown = clear_variable(resolve_unknowns(row, unknowns));
        return unknown;
    }

    fn resolve_unknowns(row: Vec<system::Term>, unknowns: Vec<Unknown>) -> Vec<system::Term> {
        let mut resolved_row: Vec<system::Term> = Default::default();
        for term in row {
            if term.pos == RIGHT {
                resolved_row.push(term);
                continue;
            }
            if term.constant == 0.0 {
                continue;
            }
            if is_unknown_to_resolve(unknowns.clone(), term.clone()) {
                let pos: usize = unknowns
                    .clone()
                    .into_iter()
                    .position(|unknown| unknown.variable == term.variable)
                    .unwrap();
                let constant = (term.constant * unknowns[pos].constant) * NEGATIVE;
                let variable = "=".to_string();
                let pos = RIGHT;
                let solved_term = system::build_term(constant, variable, pos);
                resolved_row.push(solved_term);
            } else {
                resolved_row.push(term);
            }
        }
        resolved_row.sort_by_key(|term| term.pos);
        return resolved_row;
    }

    fn clear_variable(row: Vec<system::Term>) -> Unknown {
        let mut constant: f64 = Default::default();
        let mut variable: String = Default::default();
        let mut divider: f64 = Default::default();

        for term in row {
            if term.pos == RIGHT {
                constant += term.constant;
            } else {
                variable = term.variable;
                divider = term.constant;
            }
        }
        constant /= divider;
        let unknown = Unknown { variable, constant };
        return unknown;
    }

    fn is_unknown_to_resolve(unknowns: Vec<Unknown>, term: system::Term) -> bool {
        unknowns
            .into_iter()
            .any(|unknown| unknown.variable == term.variable)
    }
}
