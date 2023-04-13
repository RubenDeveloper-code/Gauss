pub const RIGHT: i8 = 1;
pub const LEFT: i8 = -1;

pub const POSITIVE: f64 = 1.0;
pub const NEGATIVE: f64 = -1.0;

pub const STR_ZERO: &str = "0.0";
pub mod equation {
    use super::*;
    use std::{collections::HashSet, ops::Add, ptr::eq};
    #[derive(Debug, Clone)]
    pub struct Term {
        constant: f64,
        variable: String,
        pos: i8,
    }
    pub struct Unknown {
        constant: f64,
        variable: String,
    }

    pub fn equation2array(system: Vec<Vec<u8>>) -> Vec<Vec<Term>> {
        get_fixed_system_array(get_system_array(system))
    }

    fn get_system_array(system: Vec<Vec<u8>>) -> Vec<Vec<Term>> {
        let mut array: Vec<Vec<Term>> = Default::default();
        for equation in system.into_iter() {
            array.push(get_system_row(equation));
        }
        return array;
    }

    fn get_system_row(equation: Vec<u8>) -> Vec<Term> {
        let mut terms: Vec<Term> = Default::default();
        let mut sign: f64 = POSITIVE;
        let mut variable: String = Default::default();
        let mut str_const: String = Default::default();
        let mut pos: i8 = LEFT;

        for (it, value) in equation.clone().into_iter().enumerate() {
            if utilities::check_if_is_sign(value) {
                sign = utilities::convert_sign_to_value(value);
                if value == b'=' {
                    pos = RIGHT;
                }
                continue;
            }

            if value.is_ascii_digit() {
                str_const.push(value as char);
            } else if value.is_ascii_alphabetic() {
                variable.push(value as char)
            }

            if equation_utilities::term_is_ready(equation.clone(), it) {
                terms.push(equation_utilities::get_term(
                    str_const.clone(),
                    variable.clone(),
                    pos,
                    sign,
                ));
                variable.clear();
                str_const.clear();
            }
        }
        terms = equation_utilities::order_terms(terms.clone());
        return terms;
    }

    fn get_fixed_system_array(array: Vec<Vec<Term>>) -> Vec<Vec<Term>> {
        let variables: HashSet<String> = equation_utilities::get_variables_in_array(array.clone())
            .clone()
            .into_iter()
            .collect();

        let mut fixed_array: Vec<Vec<Term>> = Default::default();
        for row_of_terms in array.into_iter() {
            let mut fixed_row_of_terms: Vec<Term> = row_of_terms.clone();

            let vars_in_row_of_terms =
                equation_utilities::get_variables_in_row(row_of_terms.clone());
            let no_present_variables_in_row = &variables - &vars_in_row_of_terms;

            if !no_present_variables_in_row.is_empty() {
                fixed_row_of_terms =
                    get_fixed_row_of_terms(row_of_terms, no_present_variables_in_row);
            }
            fixed_array.push(fixed_row_of_terms.clone());
        }
        return fixed_array;
    }

    fn get_fixed_row_of_terms(row_of_terms: Vec<Term>, npv_in_row: HashSet<String>) -> Vec<Term> {
        let mut fixed_row_of_terms = row_of_terms;
        for variable in npv_in_row {
            fixed_row_of_terms.push(equation_utilities::get_term(
                STR_ZERO.to_string(),
                variable,
                LEFT,
                POSITIVE,
            ));
        }
        return equation_utilities::sort_terms_in_row(fixed_row_of_terms);
    }

    pub mod equation_utilities {
        use super::*;
        pub fn get_array_dimentions(array: Vec<Vec<Term>>) -> (usize, usize) {
            let mut n: usize = 0;
            let mut m: usize = 0;
            for (it, row) in array.clone().into_iter().enumerate() {
                if row.len() > n {
                    n = it;
                }
            }
            m = array.len();
            return (n, m);
        }
        pub fn get_variables_in_array(array: Vec<Vec<Term>>) -> Vec<String> {
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
        pub fn get_variables_in_row(row_of_terms: Vec<Term>) -> HashSet<String> {
            return row_of_terms
                .clone()
                .into_iter()
                .map(|term| term.variable.clone())
                .collect();
        }
        pub fn term_is_ready(equation: Vec<u8>, it: usize) -> bool {
            if utilities::is_last_element(equation.len(), it) {
                true
            } else {
                let next = equation[it + 1];
                if utilities::check_if_is_sign(next) {
                    true
                } else {
                    false
                }
            }
        }

        pub fn get_term(str_const: String, variable: String, pos: i8, sign: f64) -> Term {
            let variable = get_var(variable);
            let constant = get_constant(str_const, sign);

            let term = Term {
                constant,
                variable,
                pos,
            };
            term
        }

        pub fn get_constant(str_const: String, sign: f64) -> f64 {
            if str_const.is_empty() {
                return 1.0 * sign;
            } else {
                return str_const.parse::<f64>().unwrap() * sign;
            }
        }

        pub fn get_var(mut variable: String) -> String {
            if variable.is_empty() {
                variable = String::from("=");
            }
            return variable;
        }

        pub fn order_terms(terms: Vec<Term>) -> Vec<Term> {
            let new_terms = terms
                .clone()
                .into_iter()
                .map(|term| swap_pos_term(term))
                .collect::<Vec<Term>>();

            return sort_terms_in_row(new_terms);
        }

        pub fn swap_pos_term(term: Term) -> Term {
            let mut swapped_term = term.clone();
            if term.variable == "=" && term.pos == LEFT {
                swapped_term.constant *= NEGATIVE;
                swapped_term.pos = RIGHT;
            } else if term.variable != "=" && term.pos == RIGHT {
                swapped_term.constant *= NEGATIVE;
                swapped_term.pos = LEFT;
            }
            return swapped_term;
        }

        pub fn sort_terms_in_row(row_of_terms: Vec<Term>) -> Vec<Term> {
            let mut sorted_row_of_terms = row_of_terms;
            sorted_row_of_terms.sort_by_key(|term| term.variable.clone());
            sorted_row_of_terms.sort_by_key(|term| term.pos);
            return sorted_row_of_terms;
        }
    }
}

mod utilities {
    use super::*;
    pub fn check_if_is_sign(ch: u8) -> bool {
        if ch.eq(&b'+') || ch.eq(&b'-') || ch.eq(&b'=') {
            true
        } else {
            false
        }
    }
    pub fn convert_sign_to_value(ch: u8) -> f64 {
        let mut sign: f64 = 0.0;
        if ch.eq(&b'+') {
            sign = POSITIVE;
        } else if ch.eq(&b'-') {
            sign = NEGATIVE;
        } else if ch.eq(&b'=') {
            sign = POSITIVE;
        }
        sign
    }
    pub fn is_last_element(len: usize, pos: usize) -> bool {
        if pos == len - 1 {
            true
        } else {
            false
        }
    }
}
