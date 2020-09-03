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

## Adding a function for the function management system.
Let's say we want to add a function which sets a pin which is connected to a led.
First we will have to edit src/server/func_man.rs file. In there there is a function called init which looks like this:

```rust
pub fn init() -> Serial<USART2, (PA2<Alternate<AF7, Input<Floating>>>, PA3<Alternate<AF7, Input<Floating>>>)>{
    ...

    let mut gpioa = dp.GPIOA.split(&mut ahb2);

    ...
    
    let led1 = gpioa.pa5.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
    let led2 = gpioa.pa6.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
    let led3 = gpioa.pa7.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
    let led4 = gpioa.pa8.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
    
    // Replacing the Shared Peripheral
    // Also change here to if you changed SharedPeripherals
    cortex_m::interrupt::free(|cs|{
        SHARED_PER.borrow(cs).replace(Some(
            SharedPeripherals{led1,led2,led3,led4}
        ));
    });
    ...
}
```
With this hal crate we will write this line to add a push pull output pin object.
```rust
let led5 = gpioa.pa9.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
```
Now we should add this object to our SharedPeripherals struct like this. SharedPeripherals is also in "func_man.rs".
```rust
/// Change Here If An External Function Needs To Access Peripheral Data
pub struct SharedPeripherals{
    pub led1:PA5<Output<PushPull>>,
    pub led2:PA6<Output<PushPull>>,
    pub led3:PA7<Output<PushPull>>,
    pub led4:PA8<Output<PushPull>>,
    pub led5:PA9<Output<PushPull>>, // new object added
} 
```
After this we should change how we crate out SHARED_PER object. Which is the object we will use to share our peripherals. Please note that the fields here are public.

```rust
// Replacing the Shared Peripheral
    // Also change here to if you changed SharedPeripherals
    cortex_m::interrupt::free(|cs|{
        SHARED_PER.borrow(cs).replace(Some(        // added led5 here
            SharedPeripherals{led1,led2,led3,led4,led5}
        ));
    });
```
This is how SHARED_PER variable is initialized in the cortex_m::interrupt::free function which takes a closure. In this closure SHARED_PER is borrowed and replaced by the initial object which is an instance of the struct SharedPeripheral.  

Now that we made our peripheral object accessible we should start writing our function in func_man.rs.
Every function that is added should have this signature;
```rust
/// FuncId = "new_func"
pub fn func(args:&Vec::<u8>) -> Result<(),Error>{
    ...
}
```
Here Error is the enum defined in pus/src/error.rs. All the return types should be in this format.
Also if a shared variable is going to used cortex_m::interrupt::free function will have to be used to change the shared objects safely.
This will be our function.
```rust
/// FuncId = "new_led"
pub fn new_led(args:&Vec::<u8>) -> Result<(),Error>{
    cortex_m::interrupt::free(|cs| -> Result<(),Error> {
        if args[0] != 0 {
            SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led5.set_high()?;
            Ok(())
        }
        else {
            SHARED_PER.borrow(cs).try_borrow_mut()?.as_mut()?.led5.set_low()?;
            Ok(())
        } 
    })
}
```
As seen cortex_m::interrupt::free function is being used here. What it does is get a closure (lambda exp.) as input and executes it safely.
```rust
cortex_m::interrupt::free(|cs| -> Result<(),Error> {
...
})
```
Most of our functions will have this form. Here cs a variable to show the scope of the critical section and it will be needed to borrow our shared variables.
Some of the error types can be converted to error type defined in the pus crate so "?" can be used to propagate the error. Finally set_low/high is being called on our object to change the pin output. All the errors are propagated and if the function executes succesfully unit type Ok(()) should be returned.   

Final step is to add the function name and function pointer to the function map. In server.rs there will be a function called handle packets.
The function name and pointer should be added like this:
``` rust
// Function reads the packet and parses it and sends parsed packet.
pub fn handle_packets() -> ! {
    /* FUNCTION MAP AREA START */ 
    let funcs:HashMap<FuncId,fn(&Vec::<u8>)->Result<(),Error>> = pus::map!(
        create_func_id("turn_led") => turn_led as fn(&Vec::<u8>)->Result<(),Error>,
        create_func_id("set_led") => set_led as fn(&Vec::<u8>)->Result<(),Error>,
        create_func_id("new_led") => new_led as fn(&Vec::<u8>)->Result<(),Error> // new line here
    );
    /* FUNCTION MAP AREA END */
    ...
}

``` 
Here helper function "create_func_id("new_led")" function is used to create a function id from a string and new_led is casted to a function pointer type
fn(&Vec::<u8>)->Result<(),Error>.
These are the instructions to add a new function to the service. You can test is with the current client program. For example with this command "cargo run new_led 1" you can set the led but the client code currently only takes the funtion name and the arguments one by one as a byte array. With the current client code the output will be like this.
```
selman@selman-G3-3590:~/Documents/Prust/client$ cargo run new_led 0
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/x86_64-unknown-linux-gnu/debug/client new_led 0`
Success
PrimaryHeader { ver_no: 0, type_flag: false, sec_header_flag: true, apid: 2, seq_flags: (true, true), packet_name: 0, data_len: 14 }
selman@selman-G3-3590:~/Documents/Prust/client$ cargo run new_led 1
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/x86_64-unknown-linux-gnu/debug/client new_led 1`
Success
PrimaryHeader { ver_no: 0, type_flag: false, sec_header_flag: true, apid: 2, seq_flags: (true, true), packet_name: 0, data_len: 14 }
```