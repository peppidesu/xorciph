pub mod lib {
    use std::io;
    use std::io::Read;
    use std::fs;
    use std::io::Write;    

    pub fn read_piped_bytes() -> Vec<u8> {
        io::stdin().bytes().map(|x| x.unwrap()).collect()
    }

    pub fn read_file_bytes(path: &str) -> Vec<u8> {
        fs::read(path) 
            .expect("Failed to read input file.")
    }

    pub fn bytes_to_stdout(bytes: &[u8]) -> io::Result<()> {
        io::stdout().write_all(bytes)
    }
    
    pub fn encrypt(key: &String, shellcode: &[u8]) -> Vec<u8> {        
        shellcode.iter()
            .zip(key.as_bytes().iter().cycle())
            .map(|(a, b)| a ^ b)
            .collect()
    }

    pub fn print_hex(bytes: &Vec<u8>) {
        for b in bytes {
            print!("{:x}", b)
        }
        println!()
    }

    pub fn write_bytes_to_file(result: &[u8], path: &str) -> io::Result<()> {
        let mut file = fs::File::create(path)?;
        file.write_all(result)
    }
}

