targets=( aarch64-apple-darwin x86_64-apple-darwin aarch64-unknown-linux-musl x86_64-unknown-linux-musl ) # x86_64-pc-windows-gnu )

rm -r ./target/publish
mkdir "./target"
mkdir "./target/publish"
for target in "${targets[@]}"
do
  echo "Building primes-Rust for $target"
  cargo build -r --target "$target"
  strip "./target/$target/release/primes-rust"
  tar cvzf "./target/publish/primes-rust-$target.tar.gz" "./target/$target/release/primes-rust"
done
