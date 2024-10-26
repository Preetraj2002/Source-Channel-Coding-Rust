// G(2^4) => irr poly = 10011 or x^4 + x + 1
// G(2^5) => irr poly = 100101 or x^5 + x^2 + 1

#![allow(dead_code)]

// TODO - Hard-code all irr for different fields

pub static IRR_POLY: u64 = 0b0010_0101;

fn most_significant_non_zero_bit(byte: u64) -> u64 {
    if byte == 0 {
        return 0; // No non-zero bits if the byte is 0
    }

    let leading_zeros = byte.leading_zeros();
    // no. of bits that can be stored
    let capacity = 64; // TODO - genelarise the capacity: no. of bits 32, 64, ...
    return (capacity - 1) - leading_zeros as u64;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiniteField {
    pub size: u32,                 // Size of the field (e.g., 2^n)
    pub characteristic: u32,       // Characteristic of the field (e.g., 2 for GF(2^n))
    pub primitive_polynomial: u64, // Primitive polynomial for modular reduction
}

impl FiniteField {
    // Constructor for the FiniteField
    pub fn new(characteristic: u32, degree: u32, primitive_polynomial: u64) -> FiniteField {
        FiniteField {
            size: 2_u32.pow(degree),
            characteristic,
            primitive_polynomial,
        }
    }

    // TODO - Function to create an element in this field
    pub fn create_element(&self, value: u64) -> Element {
        Element {
            value,
            field: self.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    pub value: u64,         // Binary representation of the field element (for GF(2^n))
    pub field: FiniteField, // The field to which this element belongs
}

impl Element {
    // Function for field addition (XOR operation in GF(2^n))
    pub fn add(&self, other: &Element) -> Element {
        Element {
            value: self.value ^ other.value,
            field: self.field.clone(),
        }
    }

    pub fn reduce(&self) -> Element {
        let mut res: u64 = self.value;

        // Find the pos of leading term in irr poly

        let len = if let Some(pos) = (0..64).rev().find(|&i| (IRR_POLY >> i) & 1 == 1) {
            // TODO - generalise the size 32,64 ...
            pos
        } else {
            0
        };
        // Find the leading bit's position in the current ele
        let mut lead_pos = most_significant_non_zero_bit(res);

        while lead_pos >= len {
            // Calculate the number of left shifts needed
            let shift: u64 = lead_pos - len;

            // Shift the irreducible polynomial and XOR it with the current result
            res ^= IRR_POLY << shift;

            // Recalculate the leading bit's position
            lead_pos = most_significant_non_zero_bit(res);
        }

        return self.field.create_element(res);
    }

    // Function for field multiplication with reduction
    pub fn multiply(&self, other: &Element) -> Element {
        let mut result: u64 = 0;

        // reduce the input
        let mut a = (self.reduce()).value;
        let mut b = (other.reduce()).value;

        while b > 0 {
            if b & 1 != 0 {
                result ^= a;
            }
            b >>= 1;
            a <<= 1;

            // Perform modular reduction using the primitive polynomial
            if a & 0b10000 != 0 {
                // Check if the degree is 4 or higher
                a ^= self.field.primitive_polynomial;
            }
        }

        // return the reduced res
        return Element {
            value: result,
            field: self.field.clone(),
        }
        .reduce();
    }

    pub fn power(&self, exp: usize) -> Element {
        let mut res = self.field.create_element(1);
        let mut pow = exp;
        let mut base = self.clone();

        if pow == 0 {
            res = self.field.create_element(1);
            return res;
        }

        while pow > 0 {
            if pow % 2 == 1 {
                res = res.multiply(&base);
            }

            base = base.multiply(&base); // square the base
            pow /= 2;
        }

        res
    }

    pub fn inverse(&self) -> (Element, usize) {
        let zero = self.field.create_element(0);
        let one = self.field.create_element(1);
        let mut inv = zero.clone();
        let mut pw = 0;

        for i in 0..self.field.size - 1 {
            inv = self.field.create_element(1 << i);
            pw = i as usize;

            if self.multiply(&inv) == one.clone() {
                // println!("{}",inv.to_poly_str());
                return (inv, pw);
            }
        }

        // if self == &zero
        return (inv, pw);
    }

    // TODO - write divide function

    // Print as a polynomial form for human-readable output
    pub fn to_poly_str(&self) -> String {
        let mut poly = String::new();

        // no. of bits in a vars

        let capacity = 64; // Generalise this 32,64 ...

        for i in (0..capacity).rev() {
            if (self.value & (1 << i)) != 0 {
                if !poly.is_empty() {
                    poly.push_str(" + ");
                }
                if i == 0 {
                    poly.push_str("1");
                } else if i == 1 {
                    poly.push_str("a");
                } else {
                    poly.push_str(&format!("a^{}", i));
                }
            }
        }
        if poly.is_empty() {
            poly.push_str("0");
        }
        poly
    }

    pub fn bin_str(&self) -> String {
        return format!("{:08b}", self.value);
    }

    pub fn decimal_str(&self) -> String {
        return self.value.to_string();
    }
}

// Helper function to create a zero element in the finite field
pub fn element_zero(field: &FiniteField) -> Element {
    field.create_element(0)
}

// Recursive function to calculate determinant of a matrix of polynomials
pub fn determinant(matrix: Vec<Vec<&Element>>, n: usize, field: &FiniteField) -> Element {
    let zero = field.create_element(0);
    let mut det = zero.clone();

    // Base case: single element
    if n == 1 {
        det = matrix[0][0].clone();
        return det;
    }

    for p in 0..n {
        // Create submatrix for cofactor expansion

        let mut submatrix: Vec<Vec<&Element>> = vec![vec![&zero; n - 1]; n - 1];

        for i in 1..n {
            let mut col_index = 0;

            for j in 0..n {
                if j == p {
                    continue;
                }

                submatrix[i - 1][col_index] = matrix[i][j];
                col_index += 1;
            }
        }

        let cofactor = &matrix[0][p].multiply(&determinant(submatrix, n - 1, field));

        // print!("{} + ",cofactor.to_poly_str());

        det = det.add(cofactor);
    }
    // println!("val {} at n ={} ",det.to_poly_str(),n);
    return det;
}

// TODO - Write inverse matrix function

#[derive(Debug, Clone)]
pub struct Polynomial<'gf> {
    pub coefficients: Vec<Element>, // Coefficients are elements in the finite field
    pub field: &'gf FiniteField,
}

impl<'gf> Polynomial<'gf> {
    // Create a new polynomial from coefficients
    pub fn new(coefficients: Vec<Element>, field: &'gf FiniteField) -> Polynomial<'gf> {
        Polynomial {
            coefficients: coefficients.into_iter().rev().collect(),
            field,
        } // Reverse for highest power first
    }

    // Evaluate polynomial at a given element
    pub fn evaluate(&self, element: &Element) -> Element {
        let mut result = element.field.create_element(0); // Start with 0 in the field

        for (i, coeff) in self.coefficients.iter().enumerate() {
            let term = coeff.multiply(&element.power(i));
            // println!("term : {}*({})^{} =  {}",coeff.to_poly_str(),element.to_poly_str(),i, term.to_poly_str());

            result = result.add(&term);
        }
        result
    }

    // Add two polynomials
    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let max_len = if self.coefficients.len() > other.coefficients.len() {
            self.coefficients.len()
        } else {
            other.coefficients.len()
        };
        let mut res: Vec<Element> = vec![self.field.create_element(0); max_len];
        for i in 0..max_len {
            let a = self
                .coefficients
                .get(i)
                .cloned()
                .unwrap_or_else(|| element_zero(&self.field));
            let b = other
                .coefficients
                .get(i)
                .cloned()
                .unwrap_or_else(|| element_zero(&self.field));

            res[i] = a.add(&b);
            // println!("res x^{} = {:?}", i, &res[i]);
        }

        return Polynomial::new(res, &self.field);
    }

    pub fn add_assign(&mut self, other: &Polynomial<'gf>) {
        let max_len = self.coefficients.len().max(other.coefficients.len());

        // Resize the coefficients vector if necessary
        if self.coefficients.len() < max_len {
            self.coefficients
                .resize(max_len, self.field.create_element(0));
        }

        for i in 0..max_len {
            let a = self
                .coefficients
                .get(i)
                .cloned()
                .unwrap_or_else(|| element_zero(&self.field));
            let b = other
                .coefficients
                .get(i)
                .cloned()
                .unwrap_or_else(|| element_zero(&other.field));

            self.coefficients[i] = a.add(&b);
            // Optionally print out the result for debugging
            // println!("After adding, res x^{} = {:?}", i, &self.coefficients[i]);
        }
    }

    // Returns (quotient, remainder) as a tuple
    pub fn divide(&self, other: &Polynomial<'gf>) -> (Polynomial<'gf>, Polynomial<'gf>) {
        let zero_poly = Polynomial::new(vec![self.field.create_element(0)], &self.field);

        // If the degree of the divisor is zero, return zero polynomial for both quotient and remainder
        if other.coefficients.is_empty() || other.coefficients.iter().all(|c| c.value == 0) {
            return (zero_poly.clone(), zero_poly.clone());
        }

        let mut remainder = self.clone(); // Start with the dividend (self)
        let mut quotient_coeffs: Vec<Element> = vec![]; // Store coefficients for the quotient

        while remainder.coefficients.len() >= other.coefficients.len() {
            // Leading terms for both remainder and divisor
            let lead_term_rem = remainder.coefficients.last().unwrap().clone();
            let lead_term_div = other.coefficients.last().unwrap().clone();

            // Check if the remainder leading term is zero
            if lead_term_rem.value == 0 {
                // Skip this iteration, as the leading term of the remainder is zero
                remainder.coefficients.pop(); // Remove the leading zero term
                continue;
            }

            // Calculate the quotient for the leading terms
            let quotient_term = lead_term_rem.multiply(&lead_term_div.inverse().0); // Element division

            // Degree of the remainder minus degree of the divisor gives the power of the quotient term
            let degree_diff = remainder.coefficients.len() - other.coefficients.len();

            // Prepare the term to subtract from the remainder
            let mut term = vec![self.field.create_element(0); degree_diff];
            term.push(quotient_term.clone());
            term.reverse();

            // println!("{:?}",term.reverse());

            let term_poly = Polynomial::new(term, &self.field);

            // Subtract the divisor multiplied by the quotient term from the remainder
            let subtract_poly = other.multiply(&term_poly);

            remainder.add_assign(&subtract_poly);

            // Add the quotient term to the quotient
            quotient_coeffs.insert(0, quotient_term);
        }

        // Remove leading zeroes in the remainder
        while remainder.coefficients.len() > 1
            && remainder.coefficients.last() == Some(&self.field.create_element(0))
        {
            remainder.coefficients.pop();
        }

        let quotient = Polynomial::new(quotient_coeffs, &self.field);

        // Return the quotient and remainder
        return (quotient, remainder);
    }

    pub fn to_str(&self) -> String {
        let mut str = String::new();
        for (i, ele) in self.coefficients.iter().enumerate().rev() {
            if ele != &element_zero(self.field) {
                if !str.is_empty() {
                    str.push_str(" + ");
                }
                if i == 0 {
                    str.push_str(&ele.to_poly_str());
                } else if i == 1 {
                    str.push_str(&format!("({})x", &ele.to_poly_str()));
                } else {
                    str.push_str(&format!("({})x^{}", &ele.to_poly_str(), i));
                }
            }
        }

        if str.is_empty() {
            str.push_str("0");
        }

        return str;
    }

    // Multiply two polynomials
    pub fn multiply(&self, other: &Polynomial<'gf>) -> Polynomial<'gf> {
        // Prepare a result vector of coefficients with the correct length
        let mut result_coeffs =
            vec![element_zero(self.field); self.coefficients.len() + other.coefficients.len() - 1];

        // Perform polynomial multiplication
        for (i, coeff_a) in self.coefficients.iter().enumerate() {
            for (j, coeff_b) in other.coefficients.iter().enumerate() {
                let product = coeff_a.multiply(coeff_b);
                result_coeffs[i + j] = result_coeffs[i + j].add(&product);
            }
        }
        result_coeffs.reverse();
        Polynomial::new(result_coeffs, self.field)
    }

    pub fn coeff_to_bin_vec(&self) -> Vec<usize> {
        self.coefficients
            .iter()
            .map(|coeff| coeff.value as usize) // Convert each value to binary
            .collect()
    }
}

// Function to find the inverse of a matrix (Vec<Vec<&Element>>)
pub fn inverse_matrix(matrix: Vec<Vec<&Element>>) -> Option<Vec<Vec<Element>>> {
    let n = matrix.len();

    // Augment the matrix with the identity matrix
    let mut augmented_matrix: Vec<Vec<Element>> = vec![];

    for i in 0..n {
        let mut row = vec![];
        for j in 0..n {
            row.push(matrix[i][j].clone()); // Original matrix element
        }
        // Append identity matrix to the row
        for j in 0..n {
            if i == j {
                row.push(matrix[i][0].field.create_element(1)); // Add 1 for the diagonal
            } else {
                row.push(matrix[i][0].field.create_element(0)); // Add 0 otherwise
            }
        }
        augmented_matrix.push(row);
    }

    // Apply Gaussian elimination to reduce the left part to the identity matrix
    for i in 0..n {
        // Ensure the pivot is not zero by swapping with another row
        if augmented_matrix[i][i].value == 0 {
            let mut swap_row = None;
            for j in (i + 1)..n {
                if augmented_matrix[j][i].value != 0 {
                    swap_row = Some(j);
                    break;
                }
            }
            if let Some(row_to_swap) = swap_row {
                augmented_matrix.swap(i, row_to_swap);
            } else {
                return None; // No pivot found, singular matrix
            }
        }

        // Normalize the row by the pivot element
        let pivot = augmented_matrix[i][i].clone();

        // Element.inverse() returns (Element,pw)
        let pivot_inv = pivot.inverse().0;

        for j in 0..(2 * n) {
            augmented_matrix[i][j] = augmented_matrix[i][j].multiply(&pivot_inv);
        }

        // Eliminate other rows
        for j in 0..n {
            if i != j {
                let factor = augmented_matrix[j][i].clone();
                for k in 0..(2 * n) {
                    let term = augmented_matrix[i][k].multiply(&factor);
                    augmented_matrix[j][k] = augmented_matrix[j][k].add(&term);
                }
            }
        }
    }

    // Extract the right part as the inverse matrix
    let mut inverse_matrix = vec![];
    for i in 0..n {
        inverse_matrix.push(augmented_matrix[i][n..].to_vec());
    }

    Some(inverse_matrix)
}
