use std::collections::HashMap;
use std::io;
use std::vec;

// Using huffmann lib
mod huffmann;

use bch::construct_g::find_conjugates;
use huffmann::decoder::decompress;
use huffmann::encoder::compress;
use huffmann::encoder::encoded_input;
use huffmann::frequency::count_frequency_from_file;
use huffmann::tree::build_huffmann_tree;
use huffmann::tree::generate_huffmann_codes;
use huffmann::utils::get_file_size;

// Using bch lib
mod bch;
use bch::construct_g;
use bch::encoding_utils;
use bch::poly_over_gf;
use construct_g::find_generator;
use encoding_utils::create_lookup_table;
use encoding_utils::divide_into_chunks;
use poly_over_gf::determinant;
use poly_over_gf::inverse_matrix;
use poly_over_gf::Element;
use poly_over_gf::FiniteField;
use poly_over_gf::Polynomial;
use poly_over_gf::IRR_POLY;

fn main() {
    println!("Enter your Encoding Choice: ");
    println!("[1] Source Coding - Huffman code");
    println!("[2] Channel Coding - Binary BCH code");

    let choice: i32;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Not a valid string");
    choice = input.trim().parse().expect("Not a valid number");

    println!("Number is: {}", choice);

    if choice == 1 {
        println!("Source Coding");
        println!("Do you want to compress or decompress: ");
        println!("1. Compress a file");
        println!("2. Decompress a file");
        let comp_or_decomp: i32;
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Not a valid string");
        comp_or_decomp = input.trim().parse().expect("Not a valid number");
        if comp_or_decomp == 1 {
            let mut input_file = String::new();
            println!("Enter the input file name: ");
            io::stdin()
                .read_line(&mut input_file)
                .expect("Failed to get input file");
            let input_file = input_file.trim();

            println!("Enter the compressed output file name: ");
            let mut out_file = String::new();
            io::stdin()
                .read_line(&mut out_file)
                .expect("Failed get output file");
            let out_file = out_file.trim();
            match count_frequency_from_file(input_file) {
                Ok(frequency_map) => {
                    if let Some(huffmann_tree_root) = build_huffmann_tree(frequency_map) {
                        let mut huffmann_codes = HashMap::new();
                        generate_huffmann_codes(
                            &Some(huffmann_tree_root),
                            String::new(),
                            &mut huffmann_codes,
                        );

                        let input_content =
                            std::fs::read_to_string(input_file).expect("Failed to read input file");
                        let encoded_data = encoded_input(&input_content, &huffmann_codes);

                        compress(encoded_data, &huffmann_codes, &out_file)
                            .expect("Failed to compress file");
                        println!("File compressed successfully");

                        let original_size =
                            get_file_size(input_file).expect("Failed to get size of file");
                        let compressed_size =
                            get_file_size(out_file).expect("Failed to get compressed file size");

                        let compression_percentage = ((original_size - compressed_size) as f64
                            / original_size as f64)
                            * 100.0;
                        println!("Original size: {} bytes", original_size);
                        println!("Compressed size: {} bytes", compressed_size);
                        println!("Compression percentage: {:.2}%", compression_percentage);
                    }
                }
                Err(e) => println!("Failed to count frequencies: {}", e),
            }
        } else if comp_or_decomp == 2 {
            let mut compressed_file = String::new();
            let mut decompressed_file = String::new();

            println!("Provide path of input file: ");
            io::stdin()
                .read_line(&mut compressed_file)
                .expect("Failed to get compressed file name");
            println!("Provide the output file name: ");
            io::stdin()
                .read_line(&mut decompressed_file)
                .expect("Failed to get decompressed file name: ");
            let compressed_file = compressed_file.trim();
            let decompressed_file = decompressed_file.trim();

            if decompress(compressed_file, decompressed_file).is_ok() {
                println!("File compressed successfully");
            } else {
                println!("Failed to decompress file");
            }
        } else {
            println!("Invalid choice");
        }
    } else {
        println!("Channel (Error Correcting) Coding: ");
        println!();

        println!("Enter your message to be encoded: ");
        let mut mssg = String::new();
        io::stdin()
            .read_line(&mut mssg)
            .expect("Not a valid message");
        mssg = mssg.trim().to_string();

        println!("Message is: {}", mssg);
        let count = mssg.len();
        println!("Total Chars: {}", count);

        // let gf = FiniteField::new(2, 4, IRR_POLY); // x^4 + x + 1
        let gf = FiniteField::new(2, 5, IRR_POLY); // x^5 + x^2 + 1     // TODO - genelarise GF

        let zero = gf.create_element(0);
        let one = gf.create_element(1);
        // let alpha = gf.create_element(2);

        // User Defined Params

        let m_pow = 5; // TODO - generalize the m pow
        let n = (2_i32.pow(m_pow) - 1) as usize;

        let t: i32 = 5; // err correcting capability    // TODO : generalise t value

        println!("Err Correcting Capability = {}", t);

        let d = 2 * t - 1;

        // TODO - Generating individual min. poly and then find generator poly using LCM

        let zero_poly = Polynomial::new(vec![zero.clone()], &gf);
        // let one_poly = Polynomial::new(vec![one.clone()], &gf);

        // find all the conjugates

        let (conj_idx, phi) = find_conjugates(t, &gf);
        println!();
        println!("all the conjugates: {:?}", conj_idx);

        // construct all the minimal polynomials

        for i in 0..t {
            println!();
            println!(
                "\nMinimal Polynomial Phi_{}(x) = {}",
                2 * i + 1,
                phi[i as usize].to_str()
            );
        }

        // construct G poly using min. poly

        let g = find_generator(&gf, conj_idx.clone());
        println!("Generator Poly: LCM of phi_i's");
        println!("g(x) = {}", g.to_str());
        let deg_g = conj_idx.len();
        println!("Deg(g(x)) = {}", deg_g);

        let k = n - deg_g;

        println!(
            "You have chosen Binary BCH code with n = {}, k = {}, t = {}, d = {}",
            n, k, t, d
        );

        let chunks = divide_into_chunks(&mssg, k);

        // Prepare Message - k bits
        // k = n - deg(g(x))

  
        // TODO - Optional : File input

        let (char_to_num, num_to_char) = create_lookup_table();
        let mut message_text = String::new();

        for chunk in chunks.iter() {
            let text = chunk.clone();

            let numbers: Vec<usize> = text
                .chars()
                .filter_map(|c| char_to_num.get(&c).cloned())
                .collect();

            // Convert the numbers into finite field elements using gf.create_element()
            let msg_vec: Vec<Element> = numbers
                .iter()
                .map(|&n| gf.create_element(n as u64)) // Convert each number into a finite field element
                .collect();

            let msg_poly = Polynomial::new(msg_vec, &gf);

            println!("Text : {}", text);
            println!("Message : {}", msg_poly.to_str());

            let codeword = g.multiply(&msg_poly);

            println!("Codeword Generated, C = {}", codeword.to_str());

            // Add err

            let len = codeword.coefficients.len();

            // induce err

            let mut errv: Vec<Element> = vec![gf.create_element(0); len];
            // Putting err at 4 positon 
            errv[0] = errv[0].add(&gf.create_element(9));
            errv[2] = errv[2].add(&gf.create_element(9));
            errv[6] = errv[6].add(&gf.create_element(9));
            errv[1] = errv[1].add(&gf.create_element(9));

            errv.reverse();

            let e = Polynomial::new(errv, &gf);

            // Add sythetic error
            let mut recv = codeword.clone();
            recv.add_assign(&e);

            println!("Err Induced: {}", e.to_str());
            println!("Received Codeword: {}", recv.to_str());

            // Decoding

            // Syndrome Calculation - Evaluate Revc Poly at a^j

            let mut s: Vec<Element> = vec![zero.clone(); 2 * t as usize];

            // j -> [1..2t]

            for j in 0..2 * t {
                let point = gf.create_element(1 << (j + 1));

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

                let det = determinant(mat_m.clone(), l, &gf);

                println!("Det(M[{}x{}]) = {}", l, l, det.to_poly_str());

                if l == 1 && det == zero {
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

            let lambda_poly = Polynomial::new(lambda, &gf);

            println!("Lambda(x) = {}", lambda_poly.to_str());

            // Find roots of err locator function

            // Do hit and trial method to find all the roots

            // TODO - Note range and write func to generate all field elements

            let mut roots: Vec<Element> = Vec::new();
            for i in 0..n {
                let point = gf.create_element(1 << i);
                let val = lambda_poly.evaluate(&point);

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
                    x_mat[j][i] = err_loc.clone().power(j + 1);
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
            let mut y: Vec<Element> = vec![zero.clone(); l];

            // X_inv x S{lx1}

            for i in 0..l {
                let mut term = gf.create_element(0);
                for j in 0..l {
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
                let (_err_loc, inv_pw) = root.inverse();
                let mut err = Polynomial::new(vec![zero.clone(); inv_pw + 1], &gf);

                // This converts err_loc into a poly
                // e.g if err_loc = a^4 => err = x^4

                err.coefficients[inv_pw] = one.multiply(&y[i]);

                e.add_assign(&err);
            }

            println!("Err Poly : {}", e.to_str());
            println!("Raw Recv : {}", recv.to_str());

            let mut cw = Polynomial::new(vec![zero.clone(); 31], &gf); // TODO - generalise the cw len 15,31,...
            cw.add_assign(&recv);
            cw.add_assign(&e);
            println!("Corrected Codeword: ");

            println!("c(x) = {}", cw.to_str());

            // Verification of the corrected codeword

            for j in 0..2 * t {
                let point = gf.create_element(1 << (j + 1));

                s[j as usize] = cw.evaluate(&point).clone();

                println!(
                    "S{} = r({})\t= {}",
                    j + 1,
                    point.to_poly_str(),
                    s[j as usize].to_poly_str()
                );
            }

            // let mut orgin_mssg = zero_poly.clone();
            // let mut remain = zero_poly.clone();

            let (orgin_mssg, _remain) = cw.divide(&g);

            println!("Chunk mssg poly = {}", orgin_mssg.to_str());

            let orgin_mssg_vec = orgin_mssg.coeff_to_bin_vec();

            println!("Chunk mssg number vec = {:?}", orgin_mssg_vec);

            let message: String = orgin_mssg_vec
                .iter()
                .filter_map(|&n| num_to_char.get(&n).cloned())
                .collect();

            // println!("Chunk text: {}", message);

            message_text.push_str(&message);
        }

        println!("Original message text: {}", message_text);
    }
}
