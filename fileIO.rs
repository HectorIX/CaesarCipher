use std::error::Error;
use std::io::prelude::*;
use std::fs::File;


// Write a message to the given file name.
// If the file does not extist, create it.
// The first argument (filename:String) is the name of the file
// and the second argument (message:String) is the message will be stored.
pub fn write_file( filename:String, message:String ) {


    // Create the File.
    let mut file = match File::create(&filename) {

        Err(failure) => panic!("System failed to create File {} because of: {}",
                        filename,
                        failure.description()
                    ),
        Ok(file) => file,
    };

    // Write the message into the file.
    match file.write_all(message.as_bytes()) {

        Err(failure) => {
                panic!("couldn't write to {}: {}", filename
                                                 , failure.description())
                },
        Ok(_) => println!("Message successfully stored into {}.", filename),
    }

}




// Read the contents of the given file name and return them as a string.
// The argument (filename:String) is the name of the file
pub fn read_file( filename:String ) -> String {


    // Open the File.
    let mut file = match File::open(&filename) {

        Err(failure) => panic!("System failed to create File {} because of: {}",
                        filename,
                        failure.description()
                    ),
        Ok(file) => file,
    };



    let mut message = String::new();

    // Read the file and store it in message.
    match file.read_to_string(&mut message) {

        Err(failure) => {
                panic!("Couldn't read from {}: {}", filename
                                                 , failure.description())
                },
        Ok(_) => println!("Message successfully read from {}.", filename),
    };

    message

}
