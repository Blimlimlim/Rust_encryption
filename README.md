# Overview

"encryption_proj" is a command line application that lets a user encrypt and decrypt files (one at a time) using a symmetric cypher.

I created this program primarily to learn and practice the Rust programming language and introduce myself to cryptography along the way.

[Youtube demo](https://youtu.be/g1f6ECsrAAY)

### Program Usage:
Use caution when using software that modifies files on your computer. Use this software at your own risk. I have mostly tested it with text files but other file types can be used.

- Run encryption_proj.exe in a terminal.
- The program will display a prompt when the user should enter information
    -   When the following menu appears, select an option by entering 0, 1, or 2
        ```
        Select an option number
        1: encrypt
        2: decrypt
        0: quit
        ```
    - `Enter the file to encrypt:` Enter the name of an existing local file or file path. A similar prompt appears for decrypting.
    - `Enter file to save encryption:` Enter the file or path to save the encrypted file to. Will create the file if it doesn't exit already. CAUTION entering an existing file will immediately delete its contents. A similar prompt appears for decrypting.
    - `Enter the 16 character key`... Enter the key you wish to use for the encryption or decryption process. An encrypted file can only be decrypted correctly using the same key used to encrypt it.


[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

I used VS Code to write and edit the code for this software. The project was managed using Cargo and tested in Windows command line.

"encryption_proj" was coded using the Rust programming language. It uses the crate (library) rust-crypto v0.2.36 which can be found at https://crates.io/crates/rust-crypto to implement its cryptographic algorithm.

# Useful Websites

- [Tech With Tim](https://www.youtube.com/@TechWithTim)
- ["The Rust Programming Language" aka "The Book"](https://doc.rust-lang.org/book/title-page.html)
- ["Crates"](https://crates.io/)

# Future Work

- Enable user to save output over input file instead of requiring two different files for encryption and decryption operations.
- Test if padding buffer with "space" bytes can cause issues for some files
