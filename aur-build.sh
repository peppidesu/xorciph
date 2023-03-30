rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/xorciph.exe bin/
cargo aur
mv ./*.tar.gz bin/