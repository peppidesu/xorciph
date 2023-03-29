#![feature(portable_simd)]
#![feature(slice_as_chunks)]
pub mod lib {
    use std::io;
    use std::io::Read;
    use std::fs;
    use std::io::Write;    
    use std::simd::u8x32;

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
        let key_cycles = 1 + (shellcode.len() / key.len());
        let key_bytes = key.as_bytes();
        let key_length = key_bytes.len();

        let shellcode_chunks =  shellcode.chunks_exact(32);           
        let mut shellcode_rem = Vec::from(shellcode_chunks.remainder());
        let shellcode_chunks: Vec<_> = shellcode_chunks.collect();
        let chunks_end = shellcode_chunks.len() * 32;

        let key_chunks = key_bytes.repeat(key_cycles);
        let key_chunks = key_chunks.chunks_exact(32);        
        let key_chunks: Vec<_> = key_chunks.collect();

        let mut result = Vec::new();
        for i in 0..shellcode_chunks.len() {
            let c = u8x32::from_slice(shellcode_chunks[i]);            
            let k = u8x32::from_slice(key_chunks[i]);
            let xor = c ^ k;
            result.extend(xor.as_array());
        }
        for i in 0..shellcode_rem.len() {
            shellcode_rem[i] ^= key_bytes[(chunks_end + i) % key_length];
        }
        result.extend(shellcode_rem);
        result
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

