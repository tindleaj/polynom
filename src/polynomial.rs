use std::fmt;

fn strip_from_end<T: PartialEq + Clone + Default>(list: Vec<T>, object: T) -> Vec<T> {
    let mut new_list = list.clone();
    let mut strip_amount: usize = 0;
    for item in list.iter().rev() {
        if *item == object {
            strip_amount += 1;
        } else {
            break;
        }
    }
    let default: T = Default::default();
    new_list.resize(list.len() - strip_amount, default);
    new_list
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
        let stripped_coefficients = strip_from_end(coefficients, 0f64);
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
        let mut a_coefficients = self.coefficients.clone();
        let mut b_coefficients = other.coefficients.clone();

        // Resize coeff vectors to the longer size
        if a_coefficients.len() < b_coefficients.len() {
            a_coefficients.resize(b_coefficients.len(), 0f64)
        } else {
            b_coefficients.resize(a_coefficients.len(), 0f64)
        }

        let new_coefficients: Vec<f64> = a_coefficients
            .iter()
            .zip(b_coefficients)
            .map(|pair| pair.0 + pair.1)
            .collect();

        Polynomial::new(new_coefficients, 'x')
    }

    pub fn sub(&self, other: Polynomial) -> Polynomial {
        let negative_coefficients: Vec<f64> = other
            .coefficients
            .iter()
            .map(|coeff| coeff * -1f64)
            .collect();
        let negative = Polynomial::new(negative_coefficients, 'x');

        self.add(negative)
    }

    pub fn multiply(&self, other: Polynomial) -> Polynomial {
        let mut new_coefficients: Vec<f64> =
            vec![0f64; self.coefficients.len() * other.coefficients.len()];

        for (i, self_coeff) in self.coefficients.iter().enumerate() {
            for (j, other_coeff) in other.coefficients.iter().enumerate() {
                new_coefficients[i + j] += self_coeff * other_coeff;
            }
        }

        Polynomial::new(new_coefficients, 'x')
    }

    pub fn evaluate_at(&self, determinate: f64) -> f64 {
        let mut sum = 0f64;
        for (degree, coeff) in self.coefficients.iter().enumerate() {
            sum += determinate.powi(degree as i32) * coeff;
        }

        sum
    }

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

            if *coeff == 0f64 {
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
    fn test_strip_from_end() {
        assert_eq!(
            strip_from_end(vec![1, 2, 0, 3, 0, 0, 0], 0),
            vec![1, 2, 0, 3]
        );
    }
    #[test]
    fn test_strip_from_end_on_polynomial() {
        let polynomial = Polynomial::new(vec![1f64, 2f64, 0f64, 3f64, 0f64], 'x');
        assert_eq!(polynomial.coefficients, vec![1f64, 2f64, 0f64, 3f64]);
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
        assert_eq!(polynomial.degree(), 3)
    }

    #[test]
    fn test_string_representation() {
        let polynomial = Polynomial::new(vec![1f64, 2f64, 0f64, 3f64], 'x');
        assert_eq!(polynomial.as_string(), String::from("f(x) = 1 + 2x + 3x^3"))
    }

    #[test]
    fn test_add() {
        let a_polynomial = Polynomial::new(vec![1f64, 2f64, 0f64, 3f64], 'x');
        let b_polynomial = Polynomial::new(vec![1f64, 2f64, 0f64, 3f64, 4f64], 'x');

        assert_eq!(
            a_polynomial.add(b_polynomial).coefficients,
            vec![2f64, 4f64, 0f64, 6f64, 4f64]
        )
    }

    #[test]
    fn test_add_negative_coefficients() {
        let a_polynomial = Polynomial::new(vec![1f64, 2f64, 0f64, 3f64], 'x');
        let b_polynomial = Polynomial::new(vec![-1f64, -2f64, 0f64, -3f64], 'x');

        assert_eq!(a_polynomial.add(b_polynomial).coefficients, vec![0f64])
    }

    #[test]
    fn test_multiply_simple() {
        let a_polynomial = Polynomial::new(vec![1f64, 2f64, 3f64], 'x');
        let b_polynomial = Polynomial::new(vec![1f64], 'x');

        assert_eq!(
            a_polynomial.multiply(b_polynomial).coefficients,
            vec![1f64, 2f64, 3f64]
        );
    }

    #[test]
    fn test_multiply() {
        let a_polynomial = Polynomial::new(vec![1f64, 2f64, 3f64], 'x');
        let b_polynomial = Polynomial::new(vec![3f64, 2f64, 1f64], 'x');

        assert_eq!(
            a_polynomial.multiply(b_polynomial).coefficients,
            vec![3f64, 8f64, 14f64, 8f64, 3f64]
        )
    }

    #[test]
    fn test_multiply_negative() {
        let a_polynomial = Polynomial::new(vec![1f64, 2f64, 3f64], 'x');
        let b_polynomial = Polynomial::new(vec![-3f64, -2f64, -1f64], 'x');

        assert_eq!(
            a_polynomial.multiply(b_polynomial).coefficients,
            vec![-3f64, -8f64, -14f64, -8f64, -3f64]
        )
    }

    #[test]
    fn test_evaluate_at_zero() {
        let polynomial = Polynomial::new(vec![1f64, 2f64, 3f64], 'x');

        assert_eq!(polynomial.evaluate_at(0f64), 1f64)
    }

    #[test]
    fn test_evaluate_at_five() {
        let polynomial = Polynomial::new(vec![1f64, 2f64, 3f64, 4f64], 'x');

        assert_eq!(polynomial.evaluate_at(5f64), 586.0)
    }

    #[test]
    fn test_evaluate_at_negative() {
        let polynomial = Polynomial::new(vec![-1f64, 2f64, -3f64, 4f64], 'x');

        assert_eq!(polynomial.evaluate_at(-5f64), -586.0)
    }

    #[test]
    fn test_subtract() {
        let a_polynomial = Polynomial::new(vec![1f64, 2f64, 3f64], 'x');
        let b_polynomial = Polynomial::new(vec![1f64, 2f64, 3f64], 'x');

        assert_eq!(a_polynomial.sub(b_polynomial).coefficients, vec![0f64])
    }

    #[test]
    fn test_double_negative_subtract() {
        let a_polynomial = Polynomial::new(vec![-1f64, -2f64, -3f64], 'x');
        let b_polynomial = Polynomial::new(vec![-3f64, -2f64, -1f64], 'x');

        assert_eq!(
            a_polynomial.sub(b_polynomial).coefficients,
            vec![2f64, 0f64, -2f64]
        )
    }

    #[test]
    fn test_negative_subtract() {
        let a_polynomial = Polynomial::new(vec![-1f64, -2f64, -3f64], 'x');
        let b_polynomial = Polynomial::new(vec![3f64, 2f64, 1f64], 'x');

        assert_eq!(
            a_polynomial.sub(b_polynomial).coefficients,
            vec![-4f64, -4f64, -4f64]
        )
    }

}
