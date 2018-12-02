use std::fs::File;
use std::io::Read;

pub fn file_as_string(filename: &str) -> Option<String> {
    let mut buffer = String::new();
    let mut f = match File::open(filename) {
        Ok(x) => x,
        Err(_) => return None,
    };
    match f.read_to_string(&mut buffer) {
        Ok(x) => x,
        Err(why) => {
            println!("could not read contents of file: {}", why);
            return None;
        }
    };
    Some(buffer)
}

pub fn str_as_u8(s: &str) -> u8 {
    s.trim().parse::<u8>().unwrap()
}

pub fn str_as_f64(s: &str) -> f64 {
    s.trim().parse::<f64>().unwrap()
}
