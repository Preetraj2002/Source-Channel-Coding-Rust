#![allow(unused)]

use std::collections::HashSet;
use std::io;
use std::vec;

mod poly_over_gf;
use poly_over_gf::determinant;
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
        let one = gf16.create_element(1);
        let alpha = gf16.create_element(2);

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

        let t: i32 = 3; // err correcting capability
        let end = 2 * t - 1;

        println!("Err Correcting Capability = {}", t);

        // initialize the min. poly 1

        // TODO - Generating individual min. poly and then find generator poly using LCM

        let zero_poly = Polynomial::new(vec![zero.clone()], &gf16);
        let one_poly = Polynomial::new(vec![one.clone()], &gf16);

        // Initialize vec of all minimal polymonial
        let mut phi: Vec<Polynomial<'_>> = vec![one_poly; t as usize];

        for i in (0..t) {
            println!("Phi_{}(x) = {:?}", 2 * i + 1, phi[i as usize].to_str());
        }

        // Note all the conjugates of the min. poly in a set

        let mut conj_idx: HashSet<u32> = HashSet::new();

        for i in (0..t) {
            let mut pw: u32 = (2 * i + 1) as u32;

            println!(
                "\nConjugates of {} = {}",
                gf16.create_element(1 << pw).to_poly_str(),
                gf16.create_element(1 << pw).reduce().to_poly_str()
            );

            loop {
                pw *= 2;
                pw = pw % 15;
                if conj_idx.contains(&pw) || pw == 0 {
                    // println!("\nElement already contained ! Stop checking for conjugates");
                    break;
                }
                conj_idx.insert(pw);
                print!(
                    "{} = {} ,",
                    gf16.create_element(1 << pw).to_poly_str(),
                    gf16.create_element(1 << pw).reduce().to_poly_str()
                );

                let root = gf16.create_element(1 << pw).reduce();
                let term = Polynomial::new(vec![one.clone(), root], &gf16);

                phi[i as usize] = phi[i as usize].multiply(&term);
            }

            println!();
            println!(
                "\nMinimal Polynomial Phi_{}(x) = {}",
                2 * i + 1,
                phi[i as usize].to_str()
            );
        }

        println!("all the conjugates: {:?}", conj_idx);
        println!();
        println!("Generator Poly: LCM of [phi_1, phi_3, phi_5]");

        let mut g = Polynomial::new(vec![one.clone()], &gf16);

        for i in &conj_idx {
            let root = gf16.create_element(1 << i);
            let term = Polynomial::new(vec![one.clone(), root], &gf16);
            // println!("Term: ({})",term.to_str());
            g = g.multiply(&term);
        }
        println!("g(x) = {}", g.to_str());
        let deg_g = &conj_idx.len();
        println!("Deg(g(x)) = {}", deg_g);

        // Prepare Message - k bits
        // k = n - deg(g(x))

        // TODO - Map user input to GF elements
        //      - Preprocess - lowercase,whitespace,other symbols
        //      - Map each char(using ASCII) into a GF
        //      - Block message into k bits chunks
        //      - Optional : File input

        // demo message
        let messg: Vec<Element> = vec![gf16.create_element(1); 5];

        let m = Polynomial::new(messg, &gf16);

        println!("Message : {}", m.to_str());

        let codeword = g.multiply(&m);

        println!("Codeword Generated, C = {}", codeword.to_str());

        // Add err

        let mut errv: Vec<Element> = vec![gf16.create_element(0); 5];
        errv[0] = errv[0].add(&one);
        // errv[2] = errv[2].add(&one);
        errv.reverse();

        let e = Polynomial::new(errv, &gf16);
        // let recv = codeword.add(&e);
        let recv = Polynomial::new(
            vec![
                one.clone(),
                zero.clone(),
                one.clone(),
                zero.clone(),
                zero.clone(),
                zero.clone(),
            ],
            &gf16,
        );

        println!("Err Induced: {}", e.to_str());
        println!("Received Codeword: {}", recv.to_str());

        // Decoding

        // Syndrome Calculation - Evaluate Revc Poly at a^j

        let mut s: Vec<Element> = vec![zero.clone(); 2 * t as usize];

        // j -> [1..2t]

        for j in 0..2 * t {
            let point = gf16.create_element(1 << (j + 1));

            s[j as usize] = recv.evaluate(&point);

            println!(
                "S{} = r({})\t= {}",
                j + 1,
                point.to_poly_str(),
                s[j as usize].to_poly_str()
            );
        }

        let mut l: usize = t as usize; // Assume no. of errors

        // Construct M matrix with syndromes s

        let mut mat_m = vec![vec![&zero; l as usize]; l as usize];
        println!("Constructing syndrome matrix, M: ");

        for i in 0..l {
            for j in 0..l {
                // print!("{}\t",(i+j));
                mat_m[i][j] = &s[i + j];
                print!("{}\t\t", mat_m[i][j].to_poly_str());
            }
            println!();
        }

        // det(M)

        let det = determinant(mat_m, l, &gf16);

        println!("Det(M[{}x{}]) = {}", l, l, det.to_poly_str());

        if det == zero {
            l -= 1;
        }

        let mut mat_m = vec![vec![&zero; l as usize]; l as usize];
        println!("Constructing syndrome matrix, M: ");

        for i in 0..l {
            for j in 0..l {
                // print!("{}\t",(i+j));
                mat_m[i][j] = &s[i + j];
                print!("{}\t\t", mat_m[i][j].to_poly_str());
            }
            println!();
        }

        // det(M)

        let det = determinant(mat_m.clone(), l, &gf16);

        println!("Det(M[{}x{}]) = {}", l, l, det.to_poly_str());

        // inverse m

        println!("Inverse of M : ");

        let temp = mat_m[0][0].clone();
        mat_m[0][0] = mat_m[1][1];

        mat_m[1][1] = &temp;


        for i in 0..l {
            for j in 0..l {
                print!("{}\t\t", mat_m[i][j].to_poly_str());
            }
            println!();
        }

        // construct S = [s_l+1 s_l+2 s_l+3 ... s_2l]^T

        let mut s1 = vec![&zero;2*l];

        print!("S : ");

        for i in 0..l{
            s1[i] = &s[l+i];
            print!("{}\t",s1[i].to_poly_str());
        }

        println!();
        
        




        // get lambda as coeff of err locator function


        let mut lambda = vec![zero.clone();l];

        // Multiply M_inv * S

        println!("Lambda Values: ");

        for i in 0..l{
            for j in 0..l{
                lambda[i] = lambda[i].add(&mat_m[i][j].multiply(s1[j]));
            }
            println!("lambda{} = {}",l-i,lambda[i].to_poly_str());
            // Note indexing of Lambda -> [l,l-1 ... 1]
        }

        println!();

        // Construct Err locator function

        lambda.push(one.clone());   // add one as const term coeff.

        let mut lambda_poly = Polynomial::new(lambda, &gf16);

        println!("Lambda(x) = {}",lambda_poly.to_str());


        // Find roots of err locator function

        // Do hit and trial method to find all the roots

        // TODO - Note range and write func to generate all field elements

        let mut roots: Vec<Element> = Vec::new();
        for i in 0..15
        {
            let mut point = gf16.create_element(1 << i);
            let mut val = lambda_poly.evaluate(&point);
            

            if val == zero{
                println!("Root found : {}",point.to_poly_str());
                roots.push(point);

            }
        }

        let mut e = zero_poly.clone();

        // Invert the roots to find the err location number
        print!("Err Location = ");
        for root in roots{

            let mut term = root.inverse();
            print!("{}, ",term.0.to_poly_str());

            // term.1 : all power of the inverse element
            // use this to convert a^i => x^i e.g. a^3 = x^3

            let mut err = Polynomial::new(vec![zero.clone();term.1+1], &gf16);
            err.coefficients[term.1] = one.clone();


            e.add_assign(&err);

        }
        println!();

        // Construct the err poly\

        println!("Err Poly : {}",e.to_str());

        println!("Corrected Codeword: ");
        let cw = recv.add(&e);
        println!("c(x) = {}",cw.to_str());


      

        // Add error locator poly with received codeword




        // TODO - implement BCH code
    }
}
