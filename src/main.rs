#![allow(unused)]

use std::collections::HashSet;
use std::io;

mod poly_over_gf;
use poly_over_gf::Element;
use poly_over_gf::FiniteField;
use poly_over_gf::Polynomial;
use poly_over_gf::IRR_POLY;

fn main() {
    println!("Enter your message to be encoded: ");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Not a valid message");
    s = s.trim().to_string();

    println!("Message is: {}", s);

    for c in s.chars() {
        println!("{}", c);
    }
    let count = s.len();
    println!("Total Chars: {}", count);

    // TODO - convert mssg into binary stream
    // TODO - break stream into block of size n

    println!("Enter your Encoding Choice: ");
    println!("1.Source Coding - Huffman code");
    println!("2.Channel Coding - Binary BCH code");

    let choice: i32;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Not a valid string");
    choice = input.trim().parse().expect("Not a valid number");

    println!("Number is: {}", choice);

    if choice == 1 {
        println!("Source Coding: ");
        // TODO - implement Huffman code
    } else {
        println!("Channel (Error Correcting) Coding: ");
        // Define the value 'a' in GF(2^4)

        let gf16 = FiniteField::new(2, 4, IRR_POLY); // x^4 + x + 1
        let zero = gf16.create_element(0);

        let a = gf16.create_element(5);

        let b = gf16.create_element(0b1100); // Represents a^3 + a^2 (binary 1100)

        // println!("{}", a.to_polynomial());
        // println!("{}", a.bin_str());
        // println!("{}", a.decimal_str());

        // println!("{:?}", b);

        // let c = gf16.create_element(0b0001);

        // println!("c{}", c.to_poly_str());

        // // Create polynomials
        // let poly1 = Polynomial::new(vec![a.clone(), c.clone()], &gf16);
        // let poly2 = Polynomial::new(vec![b.clone(), gf16.create_element(0b0000)], &gf16);

        // // Print results
        // println!("Polynomial 1: {}", poly1.to_str());
        // println!("Polynomial 2: {:?}", poly2);

        // // Evaluate a polynomial at a given element
        // let evaluation = poly1.evaluate(&a);
        // println!("Evaluation of Polynomial 1 at x + 1: {:?}", evaluation);

        // println!("Sum of P1 and P2 = {:?}", poly1.add(&poly2));
        // println!("Poly 1 {:?}", poly1);
        // println!("Poly 2 {}", poly2.to_str());

        // let's find conjugates

        let t = 3; // err correcting capability

        // initialize the min. poly 1

        // TODO - Generating individual min. poly and then find generator poly using LCM

        let phi1 = Polynomial::new(vec![zero.clone()], &gf16);
        println!("{}", phi1.to_str());

        // find conjugate of a = a^2, a^4, a^8

        // let mut conj_phi1 = Vec::new();

        // let alpha = gf16.create_element(8); // 2 = 0010 = a

        // conj_phi1.push(&alpha);

        let a = gf16.create_element(0b0001_0011);
        let b = gf16.create_element(51);
        let c = gf16.create_element(0b0001_0000_0000);


        println!("reduce a: {} = {}",a.to_poly_str(),a.reduce().to_poly_str());
        println!("reduce b: {} = {}",b.to_poly_str(),b.reduce().to_poly_str());
        println!("reduce c: {} = {}",c.to_poly_str(),c.reduce().to_poly_str());



        // Note all the conjugates of the min. poly in a set : 

        let mut conj_idx:HashSet<i32>  = HashSet::new();
        let end = 2*t -1 ;

        for i in (1..=end).step_by(2){

            let mut pw = i;

            println!("Conjugates of a^{} = {} = {}",pw, gf16.create_element(1 << pw).to_poly_str(), gf16.create_element(1 << pw).reduce().to_poly_str());

            loop {
                pw *= 2;
                pw = pw % 15;
                if conj_idx.contains(&pw) || pw == 0 {
                    // println!("\nElement already contained ! Stop checking for conjugates");
                    break;
                }
                conj_idx.insert(pw);
                print!("{} = {} ",gf16.create_element(1 << pw).to_poly_str(),gf16.create_element(1 << pw).reduce().to_poly_str());
            }
            println!();

        }

        

        // TODO - implement BCH code
    }
}
