# Simple Blink Project in Embedded Rust Using STM32L476
## Introduction
This will be a project for STM NUCLEO-L476 (STM32L476RGT6U) but this configuration can applied to any stm32l476 only except for the memory.x file.
The cargo project created will be in this directory called "blink-stm32l476". Linux system will be used in this document to explain.
## Installations
For running a template project we are going to install cargo generate command:
```terminal
cargo install cargo-generate
```
Also we are going to install some gdb and rust tools with the commands below:
```
cargo install cargo-flash
cargo install cargo-binutils
rustup component add llvm-tools-preview
sudo apt install gdb-multiarch openocd qemu-system-arm
```
``openocd`` stands for open on chip debugger. ``gdb-multiarch openocd`` command will be used to debug ARM Cortex-M programs. We also installed Qemu to emulate the chip. One last thing to do is to add the compilation targets. Check out [The Rust Embedded Book](https://docs.rust-embedded.org/book/intro/install.html) to learn which target you should use with according to your ARM Cortex-M architecture. I recommend using the User Reference and Technical Reference documents of your board to learn about the arm architecture, ram region size and position, flash memory region size and position of the board. These information will be needed to flash your program.
   
  For my board I will use this command.

```
rustup target add thumbv7em-none-eabihf

```

## Getting The Template
After installing the dependencies let's create the template project by writing the command below. After writing the command you can enter your project name to the prompt.
```
 cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
 Project Name: blink-stm32l476
 Creating project called `blink-stm32l476`...
 Done! New project created /home/selman/Documents/Prust/prust-embedded/blink-stm32l476
```
## Trying The New Project With Qemu (Optional)
First to test the project build the hello example of the template project with the our target.
```
cargo build --example hello --target thumbv7em-none-eabihf
```
To run the simulation enter this command:
```
qemu-system-arm \
-cpu cortex-m4   \
-machine lm3s6965evb   \
-nographic   \
-semihosting-config enable=on,target=native \
-kernel target/thumbv7em-none-eabihf/debug/examples/hello
```
``-cpu`` is set cortex-m4 which is our cpu of the board we are going to test.  
  `` -machine`` parameter is for the machine being used. According to [Qemu Wiki](https://wiki.qemu.org/Documentation/Platforms/ARM) Cortex-M3 and Cortex-M4 is only used at the "lm3s811evb" and "lm3s6965evb" boards.  
Even though this is not our board this we are going to select ```lm3s6965evb```  
```-semihosting-config enable=on,target=native``` this enables the emulated use the host stdout,stderr,stdin and create files on the host. 

Seeing this output and checking the return value we will see that the simulation is successful!
```
Hello, world!
echo $?
0
``` 
## Adjusting The Project Configurations
Some changes are necessary for the project because this template is for another device. Here are the changes made on the files.
### *.cargo/config* file

```
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
# uncomment ONE of these three option to make `cargo run` start a GDB session
# which option to pick depends on your system
# runner = "arm-none-eabi-gdb -q -x openocd.gdb"
# runner = "gdb-multiarch -q -x openocd.gdb"
# runner = "gdb -q -x openocd.gdb"
```
This part will be changed to:
```
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
# uncomment ONE of these three option to make `cargo run` start a GDB session
# which option to pick depends on your system
# runner = "arm-none-eabi-gdb -q -x openocd.gdb"
runner = "gdb-multiarch -q -x openocd.gdb"
# runner = "gdb -q -x openocd.gdb"
```
For linux uncomment the line specified above.  
This part will;
```
[build]
# Pick ONE of these compilation targets
# target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
target = "thumbv7m-none-eabi"    # Cortex-M3
# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
# target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
```
be changed to;
```
[build]
# Pick ONE of these compilation targets
# target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
# target = "thumbv7m-none-eabi"    # Cortex-M3
# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
```
Because we are using Cortex-M4F.
### *memory.x* file
This file is important for opening openocd and running our code on the hardware. We only change this part;
```
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* TODO Adjust these memory regions to match your device memory layout */
  /* These values correspond to the LM3S6965, one of the few devices QEMU can emulate */
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}
```
You will need to read the technical reference document for your hardware. For my hardware flash memory starts at 0x08000000 and has a length of 1024K, RAM starts at 0x20000000 and has 96K contiguous length. Normally my device is marketed as "up to 128K Ram" but given memory here needs to be contiguous from the origin, in my device 96K part and 32K (part with partition check) part have different start addresses.
Important Note: If there are memory access error after opening openocd, memory.x file might be configured wrong be sure you have read the technical reference for the device carefully.  
So finally it will look like this.
```
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* TODO Adjust these memory regions to match your device memory layout */
  /* These values correspond to the LM3S6965, one of the few devices QEMU can emulate */
  FLASH : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM : ORIGIN = 0x20000000, LENGTH = 96K
}
```
###  *openocd.cfg* file
Target device will be changed in this file. This line;
```
source [find target/stm32f3x.cfg]
```
will be changed to;
```
source [find target/stm32l4x.cfg]
```
according to the device being used.
## Understanding The Initial Code
We made all the configurations to run our code with the hardware but we haven't write a code yet. There is a *src/main.rs* file in the initial project let's understand that before we start writing our own one.  
*src/main.rs*:
```rust {rust.line-numbers}
#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    loop {
        // your code goes here
    }
}
```
First let's look at the macros. ``#![no_std]`` is for disabling the standard library because std assumes an OS which we don't have. ``#![no_main]``
is for the structure of the program, in Cortex M there is no main which can take arguments to start the program and return something, there is something called entry point instead which can be a function in the signature of ``[unsafe] fn() -> !``. This explains the ``#[entry]`` part before the main function. The entry point will be called by the reset handler after RAM being initialized.``fn main() -> !`` this signature indicates that this rust function is not expected to finish if everything goes well.  
If we look at the includes we will see a code block for selecting the panicking behavior.
*src/main.rs*:
```rust {rust.line-numbers}
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
```
These crates has functions defined as ``#[panic_handler]``. Only one can be included and some has requirements. We will not change this part. ``use panic_halt as _;``
halts the program by busy waiting in case of a panic.  
As written in the comments your code will be in the loop because this function can not return. So let's delete the ``asm::nop();`` line and write a "Hello World!" program which uses the debugger.
## Debugging
For debugging we will have to write our code first. When debugging semihosting will be enabled since the STM board will be plugged to a host, thanks to that we can use some crates to interact with the host. We will use hosts stdout to print "Hello World" in our case. Adding the ``use cortex_m_semihosting::hprintln;`` line will include the macro for writing printing to the hosts terminal. Note that if you can call this function when the board connected to the computer, otherwise the program will panic. We will add ``hprintln!("Hello World!");`` to the main loop and to simulate halting busy waiting will be done this will be enabled by adding this line:
*src/main.rs*:
```rust {rust.line-numbers}
loop {
            // simulate halting by busy waiting
}
```
After removing some comments and unused imports in the file it should look like this:  
*src/main.rs*:
```rust {rust.line-numbers}
#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

//use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    loop {
        hprintln!("Hello World!").unwrap();
        loop {
            // simulate halting by busy waiting
        }
    }
}

```
We are all set, now let's try it on the hardware!  
Plug the board to your computer, write the;
```
openocd
```
command. The output should similar to this:
```
selman@selman-G3-3590:~/Documents/Prust/prust-embedded/blink-stm32l476$ openocd
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
        http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
adapter speed: 500 kHz
adapter_nsrst_delay: 100
none separate
Info : Unable to match requested speed 500 kHz, using 480 kHz
Info : Unable to match requested speed 500 kHz, using 480 kHz
Info : clock speed 480 kHz
Info : STLINK v2 JTAG v30 API v2 SWIM v19 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 3.238345
Info : stm32l4x.cpu: hardware has 6 breakpoints, 4 watchpoints

```
Now run;
```
cargo run
```
command on another terminal.
```
selman@selman-G3-3590:~/Documents/Prust/prust-embedded/blink-stm32l476$ cargo run
   Compiling blink-stm32l476 v0.1.0 (/home/selman/Documents/Prust/prust-embedded/blink-stm32l476)
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `gdb-multiarch -q -x openocd.gdb target/thumbv7em-none-eabihf/debug/blink-stm32l476`
Reading symbols from target/thumbv7em-none-eabihf/debug/blink-stm32l476...
<core::str::Chars as core::iter::traits::iterator::Iterator>::count () at src/libcore/str/mod.rs:597
597     src/libcore/str/mod.rs: No such file or directory.
Breakpoint 1 at 0x80012ac: file /home/selman/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.12/src/lib.rs, line 562.
Breakpoint 2 at 0x80014c4: file /home/selman/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.12/src/lib.rs, line 552.
Breakpoint 3 at 0x8000ff4: file /home/selman/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-halt-0.2.0/src/lib.rs, line 32.
Breakpoint 4 at 0x8000404: file src/main.rs, line 9.
semihosting is enabled
Loading section .vector_table, size 0x400 lma 0x8000000
Loading section .text, size 0x10d0 lma 0x8000400
Loading section .rodata, size 0x348 lma 0x80014d0
Start address 0x08001246, load size 6168
Transfer rate: 15 KB/sec, 2056 bytes/write.
Note: automatically using hardware breakpoints for read-only addresses.
0x08001248 in Reset () at /home/selman/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.12/src/lib.rs:489
489     pub unsafe extern "C" fn Reset() -> ! {
(gdb) 
```
The reset handler and main are breakpoint so you will have to enter continue twice to gdb.
```
(gdb) continue
Continuing.

Breakpoint 4, main () at src/main.rs:9
9       #[entry]
(gdb) continue
Continuing.

```
Now it hit the busy waiting part. Let's take a look at our the terminal where we opened openocd.
```
...
adapter speed: 480 kHz
target halted due to debug-request, current mode: Thread 
xPSR: 0x01000000 pc: 0x08001246 msp: 0x20018000, semihosting
Hello World!
```
It worked! When debugging semihosting tools can be used as so in Rust.
## Blinking LED's
Before deploying our code to the hardware it is better to create a program that when we run on hardware we can understand it is working well without using semihosting, blinking a led with a period is a good option for this reason.  
First we will add ``stm32l4xx-hal = { version = "0.5.0", features = ["rt","stm32l4x6"]}`` to our dependencies list in *Cargo.toml* file, this crate will provide us a HAL (Hardware Abstraction Layer) API for our stm32l476 (to see crates for other devices see [this](https://github.com/rust-embedded/embedded-hal) page).
First let's see how our *src/main.rs* will be.  
*src/main.rs*:
```rust {rust.line-numbers}
#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m_rt::entry;
use stm32l4xx_hal as hal;
use crate::hal::{prelude::*,stm32};

#[entry]
fn main() -> ! {
    loop {
        let dp = stm32::Peripherals::take().unwrap(); // get the device peripheral
        let cp = cortex_m::peripheral::Peripherals::take().unwrap(); // get the cpu peripheral

        let rcc = dp.RCC.constrain();
        let mut ahb2 = rcc.ahb2;
        let flash = dp.FLASH.constrain();
        let mut acr = flash.acr;

        let mut gpioa = dp.GPIOA.split(&mut ahb2);
        let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
        
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze(&mut acr);

        // Create a delay abstraction
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        loop {
            led.set_high().unwrap();
            delay.delay_ms(1000_u32);
            led.set_low().unwrap();
            delay.delay_ms(1000_u32);
        }
    }
}

```
From the include lines you can see we added stm32l4xx crate as hal and included its preludes and the stm32 module. We want to blink the user led so can see the user manual for your hardware (in my case it is attached to the PA5 pin). Now we should learn how to use the HAL. For help write ```cargo doc --open``` command and let cargo compile you all the documentations in your project. To blink the led we should set the voltage to high and low with delays. From the docs of ``stm32l4x6-hal`` ypu can see methods called ``set_high()`` and ``set_low()``. We want to use this methods but we will need do some procedures to call these methods. With the   
```rust
let dp = stm32::Peripherals::take().unwrap();
let cp = cortex_m::peripheral::Peripherals::take().unwrap();
```   
lines we will take singletons of cpu and device peripherals.In ``let mut ahb2 = rcc.ahb2;`` line we store *ahb2* which we will use later. Below two lines are for similar reasons. To create an object which we can call ``set_high()`` on we use this two lines   
```rust
let mut gpioa = dp.GPIOA.split(&mut ahb2);
let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.moder,&mut gpioa.otyper);
```
For creating a delay abstraction which we can call on to delay a specified time we will write:
```rust
let clocks = rcc.cfgr.sysclk(48.mhz()).freeze(&mut acr);
let mut delay = hal::delay::Delay::new(cp.SYST, clocks); // Create a delay abstraction
```
The last loop is pretty simple to understand with the abstractions made before.
```rust
loop {
    led.set_high().unwrap();
    delay.delay_ms(1000_u32);
    led.set_low().unwrap();
    delay.delay_ms(1000_u32);
}
## Deploying (Flashing) our program
```
Now since our program is ready let's deploy it with
```
cargo flash --chip stm32l476rgt --release
```
Cargo flash will enable us to flash the program directly to the board. The output may be like:
```
selman@selman-G3-3590:~/Documents/Prust/prust-embedded/blink-stm32l476$ cargo flash --chip stm32l476rgt --release
   Compiling cortex-m-rt v0.6.12
   Compiling stm32l4 v0.8.0
   Compiling nb v1.0.0
   Compiling void v1.0.2
   Compiling cast v0.2.3
   Compiling nb v0.1.3
   Compiling embedded-hal v0.2.4
   Compiling stm32l4xx-hal v0.5.0
   Compiling blink-stm32l476 v0.1.0 (/home/selman/Documents/Prust/prust-embedded/blink-stm32l476)
    Finished release [optimized + debuginfo] target(s) in 24.39s
    Flashing /home/selman/Documents/Prust/prust-embedded/blink-stm32l476/target/thumbv7em-none-eabihf/release/blink-stm32l476
        WARN probe_rs::config::registry > Found chip STM32L476RGTx which matches given partial name stm32l476rgt. Consider specifying its full name.
        WARN probe_rs::architecture::arm::core::m4 > Reason for halt has changed, old reason was Halted(Request), new reason is Exception
        WARN probe_rs::architecture::arm::core::m4 > Reason for halt has changed, old reason was Halted(Breakpoint), new reason is Request
        WARN probe_rs::architecture::arm::core::m4 > Reason for halt has changed, old reason was Halted(Request), new reason is Exception
     Erasing sectors ✔ [00:00:00] [########################################################################################################################]   2.00KB/  2.00KB @  25.49KB/s (eta 0s )
 Programming pages   ✔ [00:00:00] [########################################################################################################################]   2.00KB/  2.00KB @   6.93KB/s (eta 0s )
selman@selman-G3-3590:~/Documents/Prust/prust-embedded/blink-stm32l476$ echo $?
0
```
This works! You can also test it by pressing the reset button on the board.