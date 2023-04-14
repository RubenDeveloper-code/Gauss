pub mod misc_utils {
    pub fn input_float() -> f64 {
        let mut buff = String::new();
        std::io::stdin()
            .read_line(&mut buff)
            .expect("Debe ingresar un numero -_-");
        let input: f64 = buff.trim().parse().unwrap();
        return input;
    }
}
