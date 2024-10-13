// G(2^4) => irr poly = 10011 or x^4 + x + 1

use std::{clone, fmt::format, fs::create_dir, iter::Enumerate};

// TODO - Hard-code all irr for different fields

pub static IRR_POLY: u32 = 0b0001_0011;

fn most_significant_non_zero_bit(byte: u32) -> u32 {
    if byte == 0 {
        return 0; // No non-zero bits if the byte is 0
    }

    let leading_zeros = byte.leading_zeros();
    // no. of bits that can be stored
    let capacity = 32;
    return (capacity - 1) - leading_zeros as u32;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiniteField {
    pub size: u32,                // Size of the field (e.g., 2^n)
    pub characteristic: u32,      // Characteristic of the field (e.g., 2 for GF(2^n))
    pub primitive_polynomial: u32, // Primitive polynomial for modular reduction
}

impl FiniteField {
    // Constructor for the FiniteField
    pub fn new(characteristic: u32, degree: u32, primitive_polynomial: u32) -> FiniteField {
        FiniteField {
            size: 2_u32.pow(degree),
            characteristic,
            primitive_polynomial,
        }
    }

    // TODO - Function to create an element in this field
    pub fn create_element(&self, value: u32) -> Element {
        Element {
            value,
            field: self.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    pub value: u32,          // Binary representation of the field element (for GF(2^n))
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
        let mut res: u32 = self.value;

        // Find the pos of leading term in irr poly

        let len = if let Some(pos) = (0..32).rev().find(|&i| (IRR_POLY >> i) & 1 == 1) {
            pos
        } else {
            0
        };
        // Find the leading bit's position in the current ele
        let mut lead_pos = most_significant_non_zero_bit(res);

        while lead_pos >= len {
            // Calculate the number of left shifts needed
            let shift: u32 = lead_pos - len;

            // Shift the irreducible polynomial and XOR it with the current result
            res ^= IRR_POLY << shift;

            // Recalculate the leading bit's position
            lead_pos = most_significant_non_zero_bit(res);
        }

        return self.field.create_element(res);
    }

    // Function for field multiplication with reduction
    pub fn multiply(&self, other: &Element) -> Element {
        let mut result: u32 = 0;

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
        }.reduce()
    }

    pub fn power(&self, exp: usize) -> Element {
        let mut res = self.field.create_element(1);
        let mut pow = exp;
        let mut base = self.clone();

        if pow == 0{
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

    // Print as a polynomial form for human-readable output
    pub fn to_poly_str(&self) -> String {
        let mut poly = String::new();

        // no. of bits in a vars

        let capacity = 32;


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
        return format!("{:04b}", self.value);
    }

    pub fn decimal_str(&self) -> String {
        return self.value.to_string();
    }
}

// Helper function to create a zero element in the finite field
pub fn element_zero(field: &FiniteField) -> Element {
    field.create_element(0)
}

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
}


fn main() {
    // Define GF(2^4)

    let gf16 = FiniteField::new(2, 4, IRR_POLY); // x^4 + x + 1

    let a = gf16.create_element(5);

    let b = gf16.create_element(0b1100); // Represents x^3 + x^2 (binary 1100)

    println!("{:?}", a.to_poly_str());
    println!("{:?}", b);

    // Create finite field elements
    let a = gf16.create_element(0b0010); // Represents 'a' in GF(2^4)
    let a_squared_plus_one = gf16.create_element(0b0101); // Represents a^2 + 1 which is '5'
    let one = gf16.create_element(0b0001); // Represents '1'

    // Create two polynomials (5x + 1) and (a^2 + a)x + (a + 1)
    let poly1 = Polynomial::new(vec![a_squared_plus_one.clone(), one.clone()], &gf16); // 5x + 1
    let poly2 = Polynomial::new(vec![a.clone(), a_squared_plus_one.clone()], &gf16);
    // ax + 5
}
