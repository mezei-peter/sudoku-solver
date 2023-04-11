pub trait ArgsService {
    fn new() -> Self;
    fn process(&self, args: &Vec<String>);
}

pub struct ArgsServiceImpl;

impl ArgsService for ArgsServiceImpl {
    fn new() -> ArgsServiceImpl {
        ArgsServiceImpl
    }

    fn process(&self, args: &Vec<String>) {
        let first: &String = &args[1];
        println!("{}", first);
    }
}