use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {

    let byte = c as u8;
    byte & 0b0001_1111
        
}
fn die(e: std::io:Error) {
    panic!(e);
}
fn main() {

    let _stdout = stdout().into_raw_mode().unwrap();


    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        // if the character is a control character only print byte value otherwise print the byte
        // and char value
        if c.is_control() {
            println!("{:?} \r", b);
        }else{
            println!("{:?} ({})\r", b, c);

        }
            if b == to_ctrl_byte('q'){
                break;
            }
    }
}
