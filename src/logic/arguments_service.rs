use std::fs;

pub trait ArgsService {
    fn new() -> Self;
    fn process(&self, args: &Vec<String>);
}

pub struct ArgsServiceImpl {
    defaul_grid_size: u8,
}

impl ArgsServiceImpl {
    fn handle_file_arg(&self, file_path: &String) {
        let content =
            fs::read_to_string(file_path).expect(&format!("{} - Invalid file path", file_path));
        if self.invalidate_file(&file_path, &content) {
            return;
        }
    }

    fn invalidate_file(&self, file_path: &String, content: &String) -> bool {
        if !file_path.ends_with(".sdm") {
            println!("Invalid file format. Acceptable: .sdm");
            return true;
        }
        if content.lines().count() == 1 {
            if self.defaul_grid_size.pow(2) as usize != content.len() {
                println!("Corrupted file data: length of lines are wrong.");
                return true;
            }
        }
        
        println!("File validated.");
        false
    }
}

impl ArgsService for ArgsServiceImpl {
    fn new() -> ArgsServiceImpl {
        ArgsServiceImpl {
            defaul_grid_size: 9,
        }
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
