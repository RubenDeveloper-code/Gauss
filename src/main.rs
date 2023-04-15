mod modules;
use modules::array_utils::*;
use modules::gauss::*;
use modules::misc_utils::*;
use modules::system::*;

fn main() {
    println!(
        "▄▄▄▄▄▄▄▄▄▄▄▄▄▄Ingrese sistema de ecuaciones y tecle 'ok' cuando este listo▄▄▄▄▄▄▄▄▄▄▄▄▄"
    );
    println!("Ejemplo x+y+z=0 ENTER 2x+....");
    let equation_system = misc_utils::collect_data();
    let array: Vec<Vec<system::Term>> = system::system2array(equation_system);

    array_utils::print_array(array.clone());

    match gauss::unknowns_from_system(array.clone()) {
        Some(unknowns) => gauss::print_unknows(unknowns),
        None => println!("No se puede resolver el sistema"),
    }
}
