use std::io::stdin;

pub trait InputReader {
    fn read_line() -> String;
}

pub struct InputReaderImpl;

impl InputReaderImpl {
    fn trim_crlf(s: &mut String) {
        if s.ends_with('\n') {
            s.pop();
            if s.ends_with('\r') {
                s.pop();
            }
        }
    }
}

impl InputReader for InputReaderImpl {
    fn read_line() -> String {
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Invalid string.");
        InputReaderImpl::trim_crlf(&mut s);
        s
    }
}
