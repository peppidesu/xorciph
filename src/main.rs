use clap::{Parser, Args};
use std::io;
use std::io::Read;
use std::fs;
use std::io::Write;


#[derive(Parser)]
#[command(name = "xorciph")]
#[command(author = "Pepijn Bakker (peppidesu)")]
#[command(version = "0.2.1")]
#[command(about = "An XOR cipher CLI written in Rust")]
struct Cli {        
    /// The key used for the xor cipher 
    #[arg(short, long)]    
    key: String,

    #[command(flatten)]
    input: InputArgs,
    
    /// Don't format output
    #[arg(short, long)]
    raw: bool
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct InputArgs {
    #[arg(long)]
    /// Provide shellcode from stdin
    #[arg(short, group = "input")]
    pipe: bool,
    
    /// Provide shellcode from a file
    #[arg(short, long, group = "input")]
    file: Option<String>,
}

fn main() {
    let cli = Cli::parse();    
        
    let shellcode: Vec<u8> =
        if cli.input.pipe {
            read_piped_bytes() // read directly from stdin
        } else {
            fs::read(cli.input.file.unwrap()) 
                .expect("Failed to read input file.")
        };

    let result = encrypt(&cli.key, &shellcode);
    // Todo: for the love of god think of something faster than this
    

    if cli.raw {
        bytes_to_stdout(&result)
            .expect("IO error during write to stdout"); // Todo: proper error handling        
    }
    else {
        for byte in result {
            print!("{:x}", byte)
        }
        println!()
    }
}

fn read_piped_bytes() -> Vec<u8> {
    io::stdin().bytes().map(|x| x.unwrap()).collect()
}

fn bytes_to_stdout(bytes: &Vec<u8>) -> io::Result<()> {
    let mut single_buf: [u8; 1] = [0;1]; 
    for b in bytes {
        single_buf[0] = *b;
        match io::stdout().write_all(&single_buf) {
            Ok(_) => {},
            Err(e) => { return Err(e) }
        }
    }

    std::io::stdout().flush()
        // if you manage to hit this, it is your own damn fault. not sorry.
        .expect("IO error / EOF encountered during stdout::flush");

    Ok(())
}

fn encrypt(key: &String, shellcode: &[u8]) -> Vec<u8> {
    let key_length = key.len();    
    let key_bytes = key.as_bytes();
     
    // this is why you want to learn haskell crow <3
    shellcode.iter() 
        .enumerate()
        .map(|(i, b)| b ^ key_bytes[i % key_length])
        .collect()
}