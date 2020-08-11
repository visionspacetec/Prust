# Prust
PUS-C on Rust

## How To Run This Project
TODO
## Sending A File
First enter this command with the proper baudrate
```
stty -F /dev/ttyACM0 speed 2000000 cs8 -cstopb -echo
```
then 
```
cat packet.bin > /dev/ttyACM0
```

## Tests
For testing the spp crate enter
```
cd spp
cargo test --target=x86_64-unknown-linux-gnu
```