# Prust
PUS-C on Rust

## How To Run This Project
Please refer to "prust-embedded/README.md" if you want a more detailed and more device independent explanation.
### Dependencies
Rust Nightly is required, also these are the commands you should enter to install other dependencies.
To install rust nightly assuming you have rustup;
```
rustup override set nightly
```
For cargo and rust dependencies you can type;
```
sudo apt install pkg-config libusb-1.0-0-dev
cargo install cargo-flash
cargo install cargo-binutils
rustup component add llvm-tools-preview
rustup target add thumbv7em-none-eabihf
```
For installing gdb-multiarch and openocd you can type;
```
sudo apt install gdb-multiarch openocd
```
### Compiling
```
cargo build
```
### To deploy to the board
To deploy the program to the board there are two alternatives.
First one is to use cargo flash tool like this;
```
cargo flash --bin serve --chip stm32l476rgt --release
```
Second one includes using openocd and gdb. First run openocd server;
```
openocd
```
Then after the expected message type;
```
cargo run --release
```
Now the debugger is connected but we won't need it anymore since the program is loaded to the board, so we can close both terminals after ensuring
the programs ran expectedly at the start. After pressing the Reset button on the board (black button), all is done!

## Testing PUS crate
For unit testing the spp crate enter;
```
cd pus
cargo utest

```
## Service Provider
To open the server (service provider). 
 ```
 openocd
 ```
 and run
 ```
 cargo run
 ```
Once the program is loaded you can close openocd and the gdb debugger and press the reset button of the board. The
server should be running. You can also use cargo flash to deploy the binary. 
```
cargo flash --chip stm32l476rgt --release
```
To send a query go to the /client directory and enter.  
```
cd client
cargo run turn_led 0
```
or
```
cargo run turn_led 1
```
to send a TC packet.
