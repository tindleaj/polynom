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

    pub fn indeterminate(&mut self, c: char) -> &mut Polynomial {
        self.indeterminate = c;
        self
    }

    pub fn add() {}
    pub fn sub() {}
    pub fn multiply() {}
    pub fn evaluate_at() {}
    pub fn as_string() {}
    pub fn degree() {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_strip_zeros() {
        let polynomial = Polynomial::new(vec![1f64, 2f64, 0f64, 3f64, 0f64], 'x');
        assert_eq!(polynomial.coefficients, vec![1f64, 2f64, 3f64]);
    }

}
