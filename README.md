<div align=center>

# biscuit8

*A modular CHIP-8 emulator library written in Rust with multiple implemented frontends included*

</div>

---

`biscuit8` is a modular CHIP-8 emulator library written in Rust with multiple supported and implemented frontends included. The `biscuit8` library crate provides a backend: the logic, processing, and instruction loop of a CHIP-8 emulator. Things like graphics, input, and audio are required to be implemented by the frontend, but numerous helper constructs are provided to assist with bridging the gap. This project also implements some frontends itself too:

+ [`pixels` (graphics), `winit` (window management and input), and `rodio` (audio)](biscuit8-pixels/)

Documentation is also included with every part of the public and private API for the library and each of its frontends! Pull requests and issues are always welcome and encouraged!
