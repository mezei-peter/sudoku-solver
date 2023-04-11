pub struct ArgumentsService;

impl ArgumentsService {
    pub fn new() -> ArgumentsService {
        ArgumentsService
    }

    pub fn process(&self, args: &Vec<String>) {
        let first: &String = &args[1];
        println!("{}", first);
    }
}