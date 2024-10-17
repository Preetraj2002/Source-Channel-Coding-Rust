#![allow(unused)]

use std::collections::HashSet;
use std::io;
use std::ops::AddAssign;
use std::vec;

mod poly_over_gf;
use poly_over_gf::determinant;
use poly_over_gf::inverse_matrix;
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

        // User Defined Params

        let m_pow = 4;
        let n = 2_i32.pow(m_pow) - 1;

        // Enter the err correcting capacity
        let t: i32 = 3; // err correcting capability
        let end = 2 * t - 1;

        println!("Err Correcting Capability = {}", t);

        let k = 0;
        let d = 2 * t - 1;
        println!("Enter params m = ");
        println!(
            "You have chosen Binary BCH code with n = {}, k = {}, t = {}, d = {}",
            n, k, t, d
        );

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

        let two = gf16.create_element(2);
        let three = gf16.create_element(3);
        let four = gf16.create_element(4);
        let five = gf16.create_element(5);


        // (15,5) k = 5
        let msg_vec = vec![one.clone(),two.clone(),three.clone(),four.clone(),five.clone()];
        let msg_poly = Polynomial::new(msg_vec, &gf16);
       
        println!("Message : {}", msg_poly.to_str());

        let codeword = g.multiply(&msg_poly);

        println!("Codeword Generated, C = {}", codeword.to_str());

        // Add err

        let mut errv: Vec<Element> = vec![gf16.create_element(0); 5];
        // errv[2] = errv[2].add(&gf16.create_element(9));

        // errv[2] = errv[2].add(&one);
        errv.reverse();

        let e = Polynomial::new(errv, &gf16);

        // Add sythetic error
        let mut recv = codeword.clone();
        recv.add_assign(&e);


        // Example 1
        // let recv = Polynomial::new(
        //     vec![
        //         one.clone(),
        //         one.clone(),
        //         zero.clone(),
        //         one.clone(),
        //         one.clone(),
        //         one.clone(),
        //         one.clone(),
        //         zero.clone(),
        //         one.clone(),
        //         one.clone(),
        //     ],
        //     &gf16,
        // );

        // Example 2
        // let mut recv = Polynomial::new(
        //     vec![
        //         one.clone(),
        //         zero.clone(),
        //         one.clone(),
        //         zero.clone(),
        //         zero.clone(),
        //         zero.clone(),
        //     ],
        //     &gf16,
        // );


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

        let mut inverse_m = vec![vec![zero.clone(); l as usize]; l as usize];

        // find the first non-zero det

        loop {
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

            let det = determinant(mat_m.clone(), l, &gf16);

            println!("Det(M[{}x{}]) = {}", l, l, det.to_poly_str());

            if l == 1 && det == zero
                    {
                        println!("No error found");
                        break;
                    }

            // No l errors check for l-1 errors
            if det == zero {
                l -= 1;
                // check for next iteration
                continue;
            }
            // Non zero determinant so error found
            else {
                println!("Inverse of M: ");

                // Unwrap the inverse
                inverse_m = inverse_matrix(mat_m.clone()).unwrap();

                for i in 0..l {
                    for j in 0..l {
                        print!("{}\t\t", inverse_m[i][j].to_poly_str());
                    }
                    println!();
                }

                // Since the no. of errors found
                break;
            }
        }


        // construct S = [s_l+1 s_l+2 s_l+3 ... s_2l]^T

        let mut s1 = vec![&zero; 2 * l];

        print!("S : ");

        for i in 0..l {
            s1[i] = &s[l + i];
            print!("{}\t", s1[i].to_poly_str());
        }

        println!();

        // get lambda as coeff of err locator function

        let mut lambda = vec![zero.clone(); l];

        // Multiply M_inv * S

        println!("Lambda Values: ");

        for i in 0..l {
            for j in 0..l {
                lambda[i] = lambda[i].add(&inverse_m[i][j].multiply(s1[j]));
            }
            println!("lambda{} = {}", l - i, lambda[i].to_poly_str());
            // Note indexing of Lambda -> [l,l-1 ... 1]
        }

        println!();

        // Construct Err locator function

        lambda.push(one.clone()); // add one as const term coeff.

        let mut lambda_poly = Polynomial::new(lambda, &gf16);

        println!("Lambda(x) = {}", lambda_poly.to_str());

        // Find roots of err locator function

        // Do hit and trial method to find all the roots

        // TODO - Note range and write func to generate all field elements

        let mut roots: Vec<Element> = Vec::new();
        for i in 0..n {
            let mut point = gf16.create_element(1 << i);
            let mut val = lambda_poly.evaluate(&point);

            if val == zero {
                println!("Root found : {}", point.to_poly_str());
                roots.push(point);
            }
        }

        let mut e = zero_poly.clone();

        // Invert the roots to find the err location number
        print!("Err Location = ");

        let mut x_mat = vec![vec![zero.clone(); l as usize]; l as usize];

        // let mut err_loc_arr: Vec<Element> = Vec::new();

        // Construct the error location matrix
        for (i, root) in roots.iter().enumerate() {
            let err_loc = root.inverse().0;
            print!("{}, ", err_loc.to_poly_str());
            // err_loc_arr.push(err_loc.clone());

            // term.1 : all power of the inverse element
            for j in 0..l {
                x_mat[j][i] = err_loc.clone().power(j+1);
            }
        }
        
        println!();



        println!("the X matrix");

        for i in 0..l {
            for j in 0..l {
                print!("{}\t\t", x_mat[i][j].to_poly_str());
            }
            println!();
        }


        // // Inline conversion to Vec<Vec<&Element>> using iterators
        let x_matrix: Vec<Vec<&Element>> = x_mat
            .iter() // Iterate over the rows
            .map(|row| row.iter().collect()) // Iterate over the elements in each row and collect references
            .collect(); // Collect the rows of references into a new matrix

        let inverse_x = inverse_matrix(x_matrix).unwrap_or(vec![vec![zero.clone()]]);

        println!("Inverse of X: ");

        for i in 0..l {
            for j in 0..l {
                print!("{}\t\t", inverse_x[i][j].to_poly_str());
            }
            println!();
        }

        // Finding the error magnitude

        // Init col vec[l x 1] for err mag
        let mut y : Vec<Element> = vec![zero.clone();l];

        // X_inv x S{lx1}

        for i in 0..l
        {
            let mut term = gf16.create_element(0);
            for j in 0..l{
                let prod = inverse_x[i][j].multiply(&s[j]);
                term = term.add(&prod);
            }
            
            y[i] = term.clone();
        }

        println!("Print the err magnitude vector");

        
            for j in 0..l {
                println!("{}\t\t", y[j].to_poly_str());
            }
            println!();
        

        // Construct the err poly

        for (i, root) in roots.iter().enumerate() {
            let (err_loc, inv_pw) = root.inverse();  
            let mut err = Polynomial::new(vec![zero.clone(); inv_pw + 1], &gf16);

            // This converts err_loc into a poly
            // e.g if err_loc = a^4 => err = x^4

            err.coefficients[inv_pw] = one.multiply(&y[i]);

            e.add_assign(&err);
        }

        println!("Err Poly : {}", e.to_str());
        println!("Raw Recv : {}",recv.to_str());

        let mut cw = Polynomial::new(vec![zero.clone();15], &gf16);
        cw.add_assign(&recv);
        cw.add_assign(&e);
        println!("Corrected Codeword: ");
        
        println!("c(x) = {}", cw.to_str());

        // Verification of the corrected codeword

        for j in 0..2 * t {
            let point = gf16.create_element(1 << (j + 1));

            s[j as usize] = cw.evaluate(&point);

            println!(
                "S{} = r({})\t= {}",
                j + 1,
                point.to_poly_str(),
                s[j as usize].to_poly_str()
            );
        }
        

        // TODO - implement BCH code
    }
}
