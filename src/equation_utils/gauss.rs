pub mod gauss {
    use super::super::equation::*;
    use super::super::misc_utils::*;
    pub fn unknowns_from_system(array: Vec<Vec<equation::Term>>) -> Vec<equation::Unknown> {
        return solve(array);
    }
    fn solve(array: Vec<Vec<equation::Term>>) -> Vec<equation::Unknown> {
        let (n, m) = equation::equation_utilities::get_array_dimentions(array.clone());
        let variables = equation::equation_utilities::get_variables_in_array(array.clone());
        let mut unknowns: Vec<equation::Unknown> = Default::default();
        if m - 1 != n {
            unknowns = values_for_missing_unknowns(missing_unknowns(variables, m));
        }
        return get_unknowns(apply_gauss_method(array), unknowns.clone());
    }

    fn missing_unknowns(variables: Vec<String>, m: usize) -> Vec<String> {
        let missing_unknowns = variables
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| i == m - 1)
            .map(|(_, missing_unknown)| missing_unknown)
            .collect::<Vec<String>>();
        return missing_unknowns;
    }

    fn values_for_missing_unknowns(missing_unknowns: Vec<String>) -> Vec<equation::Unknown> {
        let mut unknowns: Vec<equation::Unknown> = Default::default();
        for missing_unknown in missing_unknowns {
            if missing_unknown == "=" {
                continue;
            }
            println!("De valor a: {}", missing_unknown);
            let constant = misc_utils::input_float();
            let variable = missing_unknown;
            let unknown = equation::Unknown { constant, variable };
            unknowns.push(unknown);
        }
        return unknowns;
    }

    fn apply_gauss_method(array: Vec<Vec<equation::Term>>) -> Vec<Vec<equation::Term>> {
        let mut array = array.clone();
        let (N, M) = equation::equation_utilities::get_array_dimentions(array.clone());

        let mut diagonal: usize = 0;
        let mut n: usize = N;
        loop {
            let pivot = get_pivot(array.clone(), diagonal);
            array = equation::equation_utilities::replace_row_in_array(
                array.clone(),
                pivot.clone(),
                diagonal,
            );
            println!("{:?}", pivot);
            loop {
                let operated_row = solve_row(array.clone(), pivot.clone(), n, diagonal);
                array = equation::equation_utilities::replace_row_in_array(
                    array.clone(),
                    operated_row,
                    n,
                );
                equation::print_array(array.clone());
                println!("diagonal :{}", diagonal);
                if n - 1 <= diagonal {
                    break;
                } else {
                    n -= 1;
                }
            }
            diagonal += 1;
            n = N;
            if diagonal == N {
                break;
            }
        }
        return array;
    }
    fn solve_row(
        array: Vec<Vec<equation::Term>>, pivot: Vec<equation::Term>, n: usize, diagonal: usize,
    ) -> Vec<equation::Term> {
        let multiplier = array[n][diagonal].constant;
        let row: Vec<equation::Term> = array[n]
            .clone()
            .into_iter()
            .enumerate()
            .map(|(it, mut term)| {
                term.constant += pivot[it].constant * multiplier * NEGATIVE;
                term
            })
            .collect();
        return row;
    }
    fn get_pivot(array: Vec<Vec<equation::Term>>, diagonal: usize) -> Vec<equation::Term> {
        let pivot = array[diagonal]
            .clone()
            .into_iter()
            .map(|mut term| {
                term.constant /= array[diagonal][diagonal].constant;
                term
            })
            .collect::<Vec<equation::Term>>();
        return pivot;
    }
    fn get_unknowns(
        mut array: Vec<Vec<equation::Term>>, unknowns: Vec<equation::Unknown>,
    ) -> Vec<equation::Unknown> {
        let mut unknowns = unknowns;
        array.reverse();
        for row in array.into_iter() {
            unknowns.push(get_unknown(row, unknowns.clone()));
        }
        return unknowns;
    }
    fn get_unknown(
        row: Vec<equation::Term>, unknowns: Vec<equation::Unknown>,
    ) -> equation::Unknown {
        let unknown = clear_variable(resolve_unknowns(row, unknowns));
        return unknown;
    }

    fn resolve_unknowns(
        row: Vec<equation::Term>, unknowns: Vec<equation::Unknown>,
    ) -> Vec<equation::Term> {
        let mut resolved_row: Vec<equation::Term> = Default::default();
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
                let solved_term = equation::equation_utilities::build_term(constant, variable, pos);
                resolved_row.push(solved_term);
            } else {
                resolved_row.push(term);
            }
        }
        resolved_row.sort_by_key(|term| term.pos);
        return resolved_row;
    }

    fn clear_variable(row: Vec<equation::Term>) -> equation::Unknown {
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
        let unknown = equation::Unknown { variable, constant };
        return unknown;
    }
    fn is_unknown_to_resolve(unknowns: Vec<equation::Unknown>, term: equation::Term) -> bool {
        unknowns
            .into_iter()
            .any(|unknown| unknown.variable == term.variable)
    }
    pub fn print_unknows(unknowns: Vec<equation::Unknown>) {
        for unknown in unknowns {
            println!("{} = {}", unknown.variable, unknown.constant);
        }
    }
}
