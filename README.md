<a href="http://www.visionspace.com">
   <img src="https://www.visionspace.com/img/VISIONSPACE_HZ_BLACK_HR.png" alt="visionspace logo" title="visionspace_cicd" align="right" height="25px" />
</a>

Prust is a contraction of **P**US and **Rust**, and is the codename for the [PUS-C](https://ecss.nl/standard/ecss-e-st-70-41c-space-engineering-telemetry-and-telecommand-packet-utilization-15-april-2016/) implementation in the reliable [Rust](https://www.rust-lang.org/) programming language. 

<!-- There is a github wiki provided with explanations about how to use this project please refer to the links below for more information;
* [How To Run This Project](https://github.com/visionspacetec/Prust/wiki/How-to-Run-This-Project)
* [How to Use the Client Crate](https://github.com/visionspacetec/Prust/wiki/How-to-Use-the-Client-Crate)
* [How to Add a Function](https://github.com/visionspacetec/Prust/wiki/How-to-Add-a-Function) -->
PUS packets as Rust data-structures. Can be used to interpret tc/tm packets.   
Here are the modules and submodules for Prust:  
- [Prust](https://github.com/visionspacetec/Prust): This module. PUS-C data structures.  
- [Prust-Test](https://github.com/visionspacetec/Prust-Test): To test the whole project.
- [Prust-RTOS-Gen](https://github.com/visionspacetec/Prust-RTOS-Gen): Complete project as Stm32CubeIde project.
- [Prust-RTOS](https://github.com/visionspacetec/Prust-RTOS): RTOS application part.

# Example
An example usage can be found in [Prust-RTOS-Gen](https://github.com/visionspacetec/Prust-RTOS-Gen) for the [VST104](https://github.com/visionspacetec/VST104-Sierra)


## Contributing

If you would like help implementing a new feature or fix a bug, check out our **[Contributing](https://github.com/visionspacetec/Prust/blob/master/.github/contributing.md)** page and the **[Code of Conduct](https://github.com/visionspacetec/Prust/blob/master/.github/code_of_conduct.md)**!
