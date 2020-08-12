# Prust
PUS-C on Rust

## How To Run This Project
Please refer to "prust-embedded/README.md" if you want a more detailed and more device independent explanation.
### Dependencies
Rust Nightly is required, also these are the commands you should enter to install other dependencies.
```
cargo install cargo-flash
cargo install cargo-binutils
rustup component add llvm-tools-preview
sudo apt install gdb-multiarch openocd
rustup target add thumbv7em-none-eabihf
```
### Compiling
```
cargo build
```

## Sending and Receiving a File
First enter this command with the proper baudrate (2000000)
```
stty -F /dev/ttyACM0 speed 2000000 cs8 -cstopb -echo
```
then 
```
cat packet.bin > /dev/ttyACM0
```
to receive
```
cat /dev/ttyACM0 > packet.txt
```
Warning: With this baudrate debugger cannot be used because of speed limits of semihosting feature.

## Tests
For testing the spp crate enter
```
cd spp
cargo test --target=x86_64-unknown-linux-gnu
```