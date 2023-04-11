pub trait ArgsService {
    fn new() -> Self;
    fn process(&self, args: &Vec<String>);
}

pub struct ArgsServiceImpl;

impl ArgsServiceImpl {
    fn handle_file_arg(&self, file_path: &String) {
        println!("Input file path is: {}", file_path);
    }
}

impl ArgsService for ArgsServiceImpl {
    
    fn new() -> ArgsServiceImpl {
        ArgsServiceImpl
    }

    fn process(&self, args: &Vec<String>) {
        for i in 1..args.len() {
            if args[i].starts_with("--") {
                if args[i].eq("--file") {
                    self.handle_file_arg(&args[i + 1]);
                }
                continue;
            }

            if args[i].starts_with("-") {
                if args[i].ends_with("f") {
                    self.handle_file_arg(&args[i + 1]);
                }
                continue;
            }
        }
    }
}



