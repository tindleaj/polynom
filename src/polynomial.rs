fn strip_zeros(coefficients: Vec<f64>) -> Vec<f64> {
    let mut result = Vec::new();
    for coeff in coefficients {
        if coeff != 0f64 {
            result.push(coeff)
        }
    }

    result
}

struct Polynomial {
    coefficients: Vec<f64>,
    indeterminate: char,
}

impl Polynomial {
    pub fn new(coefficients: Vec<f64>, indeterminate: char) -> Polynomial {
        let stripped_coefficients = strip_zeros(coefficients);
        Polynomial {
            coefficients: stripped_coefficients,
            indeterminate,
        }
    }

    pub fn add(&self, other: Polynomial) -> Polynomial {
        let new_coefficients: Vec<f64> = self
            .coefficients
            .iter()
            .zip(other.coefficients)
            .map(|pair| pair.0 + pair.1)
            .collect();

        Polynomial::new(new_coefficients, 'x')
    }

    pub fn sub() {}
    pub fn multiply() {}
    pub fn evaluate_at() {}

    /// Return the polynomial represented as a String
    /// eg. f(x) = 1 + 2x + 3x^2
    pub fn as_string(&self) -> String {
        let mut terms = String::new();
        for (degree, coeff) in self.coefficients.iter().enumerate() {
            if degree == 0 {
                terms = format!("{}", coeff);
                continue;
            }

            if degree == 1 {
                terms = format!("{} + {}{}", terms, coeff, self.indeterminate);
                continue;
            }

            terms = format!("{} + {}{}^{}", terms, coeff, self.indeterminate, degree);
        }

        format!("f({}) = {}", self.indeterminate, terms)
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_strip_zeros() {
        let polynomial = Polynomial::new(vec![1f64, 2f64, 0f64, 3f64, 0f64], 'x');
        assert_eq!(polynomial.coefficients, vec![1f64, 2f64, 3f64]);
    }

    #[test]
    fn test_degree() {
        let polynomial = Polynomial::new(vec![1f64, 2f64, 0f64, 3f64], 'x');
        assert_eq!(polynomial.degree(), 2)
    }

    #[test]
    fn test_string_representation() {
        let polynomial = Polynomial::new(vec![1f64, 2f64, 0f64, 3f64], 'x');
        assert_eq!(polynomial.as_string(), String::from("f(x) = 1 + 2x + 3x^2"))
    }

    #[test]
    fn test_add_two_polynomials() {
        let a_polynomial = Polynomial::new(vec![1f64, 2f64, 0f64, 3f64], 'x');
        let b_polynomial = Polynomial::new(vec![1f64, 2f64, 0f64, 3f64], 'x');

        assert_eq!(
            a_polynomial.add(b_polynomial).coefficients,
            vec![2f64, 4f64, 6f64]
        )
    }

}
