use clap::{Parser, Args};
use xorciph::lib;


#[derive(Parser)]
#[command(name = "xorciph")]
#[command(author = "Pepijn Bakker (peppidesu)")]
#[command(version = "0.2.2")]
#[command(about = "An XOR cipher CLI written in Rust")]
struct Cli {        
    /// The key used for the xor cipher 
    #[arg(short, long)]    
    key: String,

    #[command(flatten)]
    input: InputArgs,
    
    /// Don't format output
    #[arg(short, long)]
    raw: bool,

    #[arg(short, long)]
    output: Option<String>
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct InputArgs {
    #[arg(long)]
    /// Provide data from stdin
    #[arg(short, group = "input")]
    pipe: bool,
    
    /// Provide data from a file
    #[arg(short, long, group = "input")]
    file: Option<String>,
}

fn main() {
    let cli = Cli::parse();    
        
    let shellcode: Vec<u8> =
        if cli.input.pipe {
            lib::read_piped_bytes() // read directly from stdin
        } else {
            lib::read_file_bytes(&cli.input.file.unwrap())
        };

    let result = lib::encrypt(&cli.key, &shellcode);
    
    if let Some(path) = cli.output {
        lib::write_bytes_to_file(&result, &path)
            .expect("IO error during write to file"); // Todo: proper error handling
    }
    else if cli.raw {
        lib::bytes_to_stdout(&result)
            .expect("IO error during write to stdout"); // Todo: proper error handling                
    }
    else {
        lib::bytes_to_stdout(&result)
            .expect("IO error during write to stdout");
        println!();
    }
    
}