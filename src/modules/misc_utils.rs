pub const RIGHT: i8 = 1;
pub const LEFT: i8 = -1;

pub const POSITIVE: f64 = 1.0;
pub const NEGATIVE: f64 = -1.0;

pub const STR_ZERO: &str = "0.0";
pub mod misc_utils {
    use super::*;
    pub fn collect_data() -> Vec<Vec<u8>> {
        let mut char_equations: Vec<Vec<u8>> = Default::default();

        loop {
            let mut buff = String::new();
            let mut char_equation = Default::default();

            std::io::stdin().read_line(&mut buff).unwrap();

            if buff.trim().to_lowercase().eq("ok") {
                break;
            } else {
                char_equation = buff.trim().as_bytes().to_vec();
                char_equations.push(char_equation);
            }
        }
        char_equations
    }

    pub fn input_float() -> f64 {
        let mut buff = String::new();
        std::io::stdin()
            .read_line(&mut buff)
            .expect("Debe ingresar un numero -_-");
        let input: f64 = buff.trim().parse().unwrap_or(1.0);
        return input;
    }
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
    pub fn float2print(number: f64) -> String {
        if number - number.trunc() != 0.0 {
            return float2fraction(number);
        } else {
            return number.to_string();
        }
    }
    pub fn float2fraction(number: f64) -> String {
        let sign = number.signum();
        let number = number.abs();
        let mut numerator: f64 = 1.0;
        let mut denominator: f64 = 1.0;
        let mut test: f64 = 1.0;
        let mut steps: i64 = 0;
        let max_steps = 1000000;
        while test != number {
            test = numerator / denominator;
            if test < number {
                numerator += 1.0;
            } else if test > number {
                numerator -= 1.0;
                denominator += 1.0;
            }
            steps += 1;
            if steps > max_steps {
                return format!("{:.4}", number);
            }
        }
        return format!("{}/{}", sign * numerator, denominator);
    }
}
