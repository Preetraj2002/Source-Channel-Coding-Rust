use std::collections::HashSet;
use super::poly_over_gf::{FiniteField, Polynomial};


// Note all the conjugates of the min. poly in a set


pub fn find_conjugates<'gf>(t: i32,gf: & 'gf FiniteField) -> (HashSet<u32>, Vec<Polynomial<'gf>>)
{
    let one = gf.create_element(1);
    let one_poly = Polynomial::new(vec![one.clone()], &gf);
    let mut phi: Vec<Polynomial<'_>> = vec![one_poly; t as usize];
    let mut conj_idx: HashSet<u32> = HashSet::new();
    let one = gf.create_element(1);

    for i in 0..t {
    let mut pw: u32 = (2 * i + 1) as u32;

    println!(
        "\nConjugates of {} = {}",
        gf.create_element(1 << pw).to_poly_str(),
        gf.create_element(1 << pw).reduce().to_poly_str()
    );

    loop {
        pw *= 2;
        pw = pw % 31; // TODO - generalise the power
        if conj_idx.contains(&pw) || pw == 0 {
            // println!("\nElement already contained ! Stop checking for conjugates");
            break;
        }
        conj_idx.insert(pw);
        print!(
            "{} = {} ,",
            gf.create_element(1 << pw).to_poly_str(),
            gf.create_element(1 << pw).reduce().to_poly_str()
        );

        let root = gf.create_element(1 << pw).reduce();
        let term = Polynomial::new(vec![one.clone(), root], &gf);

        phi[i as usize] = phi[i as usize].multiply(&term);
    }


  }

    return (conj_idx, phi);
}


pub fn find_generator<'gf>(gf:&FiniteField, conj_idx:HashSet<u32>) -> Polynomial<'_>
{
    let one = gf.create_element(1);

    let mut g = Polynomial::new(vec![one.clone()], &gf);

    for i in &conj_idx {
        let root = gf.create_element(1 << i);
        let term = Polynomial::new(vec![one.clone(), root], &gf);
        // println!("Term: ({})",term.to_str());
        g = g.multiply(&term);
    }
    return g;
}

