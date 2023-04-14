mod equation_utils;
use equation_utils::equation::*;
use equation_utils::gauss::*;

fn collect_data() -> Vec<Vec<u8>> {
    let mut char_equations: Vec<Vec<u8>> = Default::default();

    loop {
        let mut buff = String::new();
        let mut char_equation = Default::default();

        std::io::stdin().read_line(&mut buff).unwrap();

        if buff.trim().eq("ok") {
            break;
        } else {
            char_equation = buff.trim().as_bytes().to_vec();
            char_equations.push(char_equation);
        }
    }
    char_equations
}
fn main() {
    let equation_system = collect_data();
    let array: Vec<Vec<equation::Term>> = equation::equation2array(equation_system);
    equation::print_array(array.clone());
    let unknowns = gauss::unknowns_from_system(array.clone());
    gauss::print_unknows(unknowns);
}
