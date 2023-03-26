use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

/// for a given character in [a-z] range, returns it's byte combination with ctrl.
/// example: q is 113 (0b01110001) & 00011111 -> 00010001 (17) -> Ctrl-Q
/// example: Q is 81 (0b01010001) & 00011111 -> 00010001 (17) -> Ctrl-Q
/// this works because Ctrl sets the upper bits to 0 (which is what we're doing as well)
fn ctrl_combination(c: char) -> u8 {
    let byte = c as u8;
    // removes everything above 32
    byte & 0b0001_1111
}

fn die(e: io::Error) {
    panic!("{}", e)
}

fn main() {

    // bind this to a variable s.t. the terminal does not exit raw mode and go into cooked mode
    // using _ as a prefix to a variable name is a convention to indicate that the variable is unused
    let _stdout = stdout().into_raw_mode().unwrap(); // ignore the error here
    for byte in io::stdin().bytes() {
        match byte {
            Ok(b) => {
                let key = b as char; // Convert byte to character

                if b == ctrl_combination('q') {
                    break;
                }

                if key.is_control() {
                    println!("{:?}\r", b);
                } else {
                    println!("{:?} ({})\r", b, key);
                }
            }
            Err(e) => die(e)
        }


    }
}
