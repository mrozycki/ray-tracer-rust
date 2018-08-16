pub struct Polynomial {
    coefficients: Vec<f64>,
}

impl Polynomial {
    pub fn new(coefficients: Vec<f64>) -> Self {
        Polynomial { coefficients }
    }

    fn solve_quadratic(a: f64, b: f64, c: f64) -> Vec<f64> {
        let delta = b * b - 4.0 * a * c;
        if delta == 0.0 {
            vec!(-b / (2.0 * a))
        } else if delta > 0.0 {
            vec!((-b - delta.sqrt()) / (2.0 * a), (-b + delta.sqrt()) / (2.0 * a))
        } else {
            Vec::new()
        }
    }

    pub fn into_solutions(self) -> Vec<f64> {
        if self.coefficients.len() == 3 {
            return Self::solve_quadratic(self.coefficients[2], self.coefficients[1], self.coefficients[0]);
        }

        panic!("Solving polynomials of higher degrees is not implemented yet!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn solve_quadratic_one_solution() {
        let polynomial = Polynomial::new(vec!(1.0, -2.0, 1.0));

        assert_eq!(polynomial.into_solutions(), vec!(1.0));
    }

    #[test]
    pub fn solve_quadratic_no_solutions() {
        let polynomial = Polynomial::new(vec!(1.0, 0.0, 1.0));

        assert_eq!(polynomial.into_solutions(), Vec::new());
    }

    #[test]
    pub fn solve_quadratic_two_solutions() {
        let polynomial = Polynomial::new(vec!(-1.0, 0.0, 1.0));

        assert_eq!(polynomial.into_solutions(), vec!(-1.0, 1.0));
    }
}