use std::io;

fn main()
{
    
    println!("Enter your message to be encoded: ");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Not a valid message");
    s = s.trim().to_string();

    println!("Message is: {}",s);

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

    let choice:i32;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Not a valid string");
    choice = input.trim().parse().expect("Not a valid number");
    
    println!("Number is: {}",choice);
    
    if choice == 1
    {
        println!("Source Coding: ");
        // TODO - implement Huffman code
    }
    else
    {
        println!("Channel (Error Correcting) Coding: ");
        // TODO - implement BCH code
    }

}