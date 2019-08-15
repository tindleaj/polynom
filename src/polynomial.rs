use std::fmt;

fn strip_zeros(coefficients: Vec<f64>) -> Vec<f64> {
    let mut result = Vec::new();
    for coeff in coefficients {
        if coeff != 0f64 {
            result.push(coeff)
        }
    }

    result
}

pub struct Polynomial {
    coefficients: Vec<f64>,
    indeterminate: char,
}

impl fmt::Debug for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Polynomial {{ coefficients: {:?}, indeterminate: '{ind}', as_string: {} }}",
            self.coefficients,
            self.as_string(),
            ind = self.indeterminate
        )
    }
}

impl Polynomial {
    pub fn new(coefficients: Vec<f64>, indeterminate: char) -> Polynomial {
        let stripped_coefficients = strip_zeros(coefficients);
        if stripped_coefficients.len() == 0 {
            return Polynomial {
                coefficients: vec![0f64],
                indeterminate,
            };
        }

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
    /// # Example
    /// ```
    /// use polynom::polynomial::Polynomial;
    ///
    /// let polynomial = Polynomial::new(vec![1f64, 2f64, 3f64], 'x');
    /// assert_eq!(polynomial.as_string(), String::from("f(x) = 1 + 2x + 3x^2"))
    /// ```
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

    pub fn degree(&self) -> isize {
        // Special case zero polynomial
        if self.coefficients == vec![0f64] {
            return -1;
        }

        (self.coefficients.len() - 1) as isize
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
    fn test_zero_polynomial() {
        let polynomial = Polynomial::new(vec![0f64], 'x');
        assert_eq!(polynomial.coefficients, vec![0f64]);
    }

    #[test]
    fn test_zero_special_case_degree() {
        let polynomial = Polynomial::new(vec![0f64], 'x');
        assert_eq!(polynomial.degree(), -1)
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
    fn test_add() {
        let a_polynomial = Polynomial::new(vec![1f64, 2f64, 0f64, 3f64], 'x');
        let b_polynomial = Polynomial::new(vec![1f64, 2f64, 0f64, 3f64], 'x');

        assert_eq!(
            a_polynomial.add(b_polynomial).coefficients,
            vec![2f64, 4f64, 6f64]
        )
    }

    #[test]
    fn test_add_negative_coefficients() {
        let a_polynomial = Polynomial::new(vec![1f64, 2f64, 0f64, 3f64], 'x');
        let b_polynomial = Polynomial::new(vec![-1f64, -2f64, 0f64, -3f64], 'x');
        println!("{:?}", b_polynomial);

        assert_eq!(a_polynomial.add(b_polynomial).coefficients, vec![0f64])
    }

}
