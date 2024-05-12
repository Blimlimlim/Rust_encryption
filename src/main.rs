extern crate crypto;
/* Import modules start */
use crypto::aessafe::AesSafe128Decryptor;
use crypto::aessafe::AesSafe128Encryptor;
use crypto::symmetriccipher::BlockDecryptor;
use crypto::symmetriccipher::BlockEncryptor;
use std::fs::File;
use std::io;
use std::io::prelude::Write;
use std::io::Read;

/* Constants */
const BLOCK_SIZE: usize = 16; // length in bytes of a message that the encryptor can handle at a time

/* Main function */
fn main() -> std::io::Result<()> {

    // Selector for use in main program loop
    let mut menu_select = String::new();
    //print_main_menu();

    loop { //Main loop continues until user enters 0 at prompt
        if menu_select.trim() == "0" {
            break; // Exit loop
        }
        menu_select.clear();
        print_main_menu(); // See function to view menu contents
        io::stdin().read_line(&mut menu_select)?;

        match menu_select.trim() {
            "0" => (),
            "1" => encrypt_input()?,
            "2" => decrypt_input()?,
            _ => println!("Invalid option, select again"),
        }
    }
    Ok(())
}

/* ========================== Functions ========================== */

/* Displays menu */
fn print_main_menu() {
    println!(
        "========= Menu ========\n\
         Select an option number\n\
         1: encrypt\n\
         2: decrypt\n\
         0: quit\n\
         ======================="
    );
}

/* User enters filenames for encrypting to, storing encrypted data, and the symmetric key to use.
 * Then encrypts the file*/
fn encrypt_input() -> std::io::Result<()> {
    println!("Enter the file to encrypt:");
    let mut file_in_name = String::new();
    io::stdin().read_line(&mut file_in_name)?;
    let file_in = File::open(file_in_name.trim()).expect("Couldn't open file. Make sure path exists");// TODO return to mutable if needed

    println!("Enter file to save encryption:");
    let mut file_out_name = String::new();
    io::stdin().read_line(&mut file_out_name)?;
    loop { //prevents using same file as input and output
        if file_out_name.trim() != file_in_name.trim() {
            break;
        }
        println!(
            "Using the same file here will cause serious data loss\n\
                  Enter a different name:"
        );
        file_out_name.clear();
        io::stdin().read_line(&mut file_out_name)?;
    }
    let file_out = File::create(file_out_name.trim()).expect("Failed to create file");// TODO return to mutable if needed

    println!(
        "Enter the 16 character key you will use to encrypt and decrypt your file\n\
         Don't forget it!"
    );
    let mut key_string = String::new();
    io::stdin().read_line(&mut key_string)?;
    loop  {//prevents invalid key length
        if key_string.trim().len() == 16 {
            break;
        }
        println!(
            "Invalid key length, must be 16 characters\n\
             Enter again"
        );
        key_string.clear();
        io::stdin().read_line(&mut key_string)?;
    }   
    let key: &[u8] = key_string.trim().as_bytes();// TODO return to mutable if needed
    encrypt_file(file_in, file_out, key)?;
    println!("{} was encrypted to {}", file_in_name.trim(), file_out_name);
    Ok(())
}

/* User enters filenames for encrypting to, storing encrypted data, and the symmetric key to use.
 * Then decrypts the file */
fn decrypt_input() -> std::io::Result<()> {
    println!("Enter the file to decrypt:");
    let mut file_in_name = String::new();
    io::stdin().read_line(&mut file_in_name)?;
    let file_in = File::open(file_in_name.trim()).expect("Couldn't open file. make sure path exists");// TODO return to mutable if needed

    println!("Enter file to save decryption:");
    let mut file_out_name = String::new();
    io::stdin().read_line(&mut file_out_name)?;
    loop  {// prevents using the same file for input and output
        if file_out_name.trim() != file_in_name.trim() {
            break;
        }
        println!(
            "Using the same file here will cause serious data loss\n\
             Enter a different name:"
        );
        file_out_name.clear();
        io::stdin().read_line(&mut file_out_name)?;
    }
    let file_out = File::create(file_out_name.trim()).expect("failed to create file");// TODO return to mutable if needed

    println!(
        "Enter the 16 character key you used to encrypt this file\n"
    );
    let mut key_string = String::new();
    io::stdin().read_line(&mut key_string)?;
    loop  {
        if key_string.trim().len() == 16 {
            break;
        }
        println!(
            "Invalid key length, must be 16 characters\n\
             Enter again"
        );
        key_string.clear();
        io::stdin().read_line(&mut key_string)?;
    }
    let key: &[u8] = key_string.trim().as_bytes();// TODO return to mutable if needed
    decrypt_file(file_in, file_out, key)?;
    println!("{} was decrypted to {}", file_in_name.trim(), file_out_name);
    Ok(())
}

/* Encrypts `file_in` and stores the encrypted file in `file_out` using `key`
 * Called by `encrypt_menu` */
fn encrypt_file(mut file_in: File, mut file_out: File, key: &[u8]) -> std::io::Result<()> {
    // Create encryptor
    let encryptor = AesSafe128Encryptor::new(key);

    // Buffer to read 16 bytes from file at a time
    let mut buffer: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];

    // This loop reads 16 (BLOCK_SIZE) bytes from file_in, encrypts it, and writes the encrypted bytes to file_out
    loop {
        // Read BLOCK_SIZE number of bytes into the buffer
        let bytes_read = file_in.read(&mut buffer).expect("Couldn't read");

        // If no bytes were read, we've reached then end of the file
        if bytes_read == 0 {
            break; // exit loop
        }
        // TODO replace any string conversions with byte padding function
        let data = if bytes_read < BLOCK_SIZE {
            u8_slice_padder(&mut buffer[..], BLOCK_SIZE - bytes_read)
        } else {
            &buffer[..]
        };
       
        let mut writer_buffer: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
        // Encrypt data and put encrypted data in buffer
        encryptor.encrypt_block(data, &mut writer_buffer);
        // Wite encrypted data to the output file
        file_out.write_all(&writer_buffer)?;
        file_out.flush()?;
    }
    Ok(())
}

/* Decrypts `file_in` and stores the decrypted file in `file_out` using `key`
 * Called by `decrypt_menu` */
fn decrypt_file(mut file_in: File, mut file_out: File, key: &[u8]) -> std::io::Result<()> {
    // Create decryptor
    let decryptor = AesSafe128Decryptor::new(key);

    // Create buffer
    let mut buffer: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];

    // This loop reads 16 (BLOCK_SIZE) bytes from file_in, decrypts it, and writes the decrypted bytes to file_out
    loop {
        // Read BLOCK_SIZE number of bytes into the buffer
        let bytes_read = file_in.read(&mut buffer).expect("Failed to read bytes from file");

        // If no bytes were read, we've reached then end of the file
        if bytes_read == 0 {
            break; // exit loop
        }
        // Convert buffer to byte slice for decryption
        let data: &[u8] = &buffer[..];
        // New buffer needed so as not to have multiple mutable references to buffer
        let mut writer_buffer: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
        // Decrypt data
        decryptor.decrypt_block(data, &mut writer_buffer);
        // Write decrypted data to the output file
        file_out.write_all(&writer_buffer)?;
        file_out.flush()?;
    }
    Ok(())
}

fn u8_slice_padder(bytery: &mut [u8], difference: usize) -> &[u8] {
    let index_start = bytery.len() - difference; // Start at first unfilled index
    for i in 0..difference { // replace at each index to the last
        bytery[index_start + i] = 32;
    }
    bytery
}
