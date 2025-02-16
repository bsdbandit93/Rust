
//Hiding the warnings when compiling code 

#![allow(warnings)]
//Crates
extern crate hex;
extern crate base64;
extern crate encoding;
extern crate urlencoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use base64::{encode};
use figlet_rs::FIGfont;
use std::io;
use lliw::{Fg, Bg, Style, Reset};


fn main() { 
    //Ascii Art 
    let standard_font = FIGfont::standard().unwrap();
    let _figure = standard_font.convert("Bandit Encoder");
    assert!(_figure.is_some());
    println!("{}", _figure.unwrap());

    //Reading user input 
    println!("Please enter your payload: ");
    let mut payload = String::new();
    io::stdin().read_line(&mut payload);

    //base64 encoding 
    println!("\nBase64 encoded Payload");
    println!("{}", encode(&payload));

    //hex encoding 
    println!("\nHex encoded Payload");
    println!("{}", hex::encode(&payload));

    //url encoding 
    println!("\nUrl encoded Payload");
    println!("{}", urlencoding::encode(&payload));

}

    

