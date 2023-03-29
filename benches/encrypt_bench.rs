use std::time::Duration;

use xorciph::lib;
use rand::prelude::*;
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

fn generate_random_key(rng: &mut ThreadRng, len: usize) -> String {
    let mut key = String::new();
    for _ in 0..len {
        key.push(rng.gen());                
    }
    key
}
fn generate_random_shellcode(rng: &mut ThreadRng, len: usize) -> Vec<u8> {
    let mut shellcode: Vec<u8> = Vec::new();
    for _ in 0..len {
        shellcode.push(rng.gen());                
    }
    shellcode
}

fn encrypt_bench(c: &mut Criterion) {    
    c.bench_function("small (no simd)", |b| b.iter_batched(
        ||  {
            let mut rng = rand::thread_rng();
            (generate_random_key(&mut rng, 173), generate_random_shellcode(&mut rng, 2841))
        },
         |(key, shellcode)| { lib::encrypt(&key, &shellcode); },
         criterion::BatchSize::SmallInput
    ));

    c.bench_function("small (simd)", |b| b.iter_batched(
        ||  {
            let mut rng = rand::thread_rng();
            (generate_random_key(&mut rng, 173), generate_random_shellcode(&mut rng, 2841))
        },
        |(key, shellcode)| { lib::encrypt_simd(&key, &shellcode); },
        criterion::BatchSize::SmallInput
    ));

    c.bench_function("large (no simd)", |b| b.iter_batched(
        ||  {
            let mut rng = rand::thread_rng();
            (generate_random_key(&mut rng, 3986), generate_random_shellcode(&mut rng, 284172))
        },
        |(key, shellcode)| { lib::encrypt(&key, &shellcode); },
        criterion::BatchSize::SmallInput
    ));

    c.bench_function("large (simd)", |b| b.iter_batched(
        ||  {
            let mut rng = rand::thread_rng();
            (generate_random_key(&mut rng, 3986), generate_random_shellcode(&mut rng, 284172))
        },
        |(key, shellcode)| { lib::encrypt_simd(&key, &shellcode); },
        criterion::BatchSize::SmallInput
    ));
}

criterion_group!{
    name = benches;
    config = Criterion::default()
        .sample_size(50)
        .measurement_time(Duration::from_secs(5));        
    targets = encrypt_bench
}
criterion_main!(benches);