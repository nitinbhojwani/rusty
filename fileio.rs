use std::env;
use std::error::Error; 
use std::fs;
use std::io::prelude::*;
use std::mem;
use std::path::Path;
use std::str;

enum OpenFileMode {
    Read,
    Write,
    Append,
    Create,
    Remove
}

impl str::FromStr for OpenFileMode {
    type Err = ();

    fn from_str(s: &str) -> Result<OpenFileMode, ()> {
        match s {
            "Read" => Ok(OpenFileMode::Read),
            "Write" => Ok(OpenFileMode::Write),
            "Append" => Ok(OpenFileMode::Append),
            "Create" => Ok(OpenFileMode::Create),
            "Remove" => Ok(OpenFileMode::Remove),
            _ => panic!("Invalid Filemode '{}' provided", &s)
        }
    }
}


fn operate_on_file(path: &Path, filemode: &OpenFileMode) {
    if mem::discriminant(filemode) == mem::discriminant(&OpenFileMode::Remove) {
        match fs::remove_file(path) {
            Err(why) => panic!("couldn't remove the file at path {}: {}", path.display(),
                                                                        why.description()),
            Ok(_) => println!("Removed the file")
        };
        return;
    }
    let mut open_options = fs::OpenOptions::new();
    match filemode {
        OpenFileMode::Read => open_options.read(true),
        OpenFileMode::Write => open_options.write(true),
        OpenFileMode::Append => open_options.append(true),
        OpenFileMode::Create => open_options.write(true).create_new(true),
        _ => panic!("This filemode operation isn't supported")
    };

    let display = path.display();
    let mut file = match open_options.open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't perform the file operation {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    if mem::discriminant(filemode) == mem::discriminant(&OpenFileMode::Create) {
        println!("Created the file at path {}", display);
        return;
    }

    match filemode {
        OpenFileMode::Read => {
            let mut buf = String::new();
            match file.read_to_string(&mut buf) {
                Err(why) => panic!("couldn't read file at {}: {}", display,
                                                           why.description()),
                Ok(_) => println!("Read the content - {}", &buf),
            }
        },
        OpenFileMode::Write => {
            let content = b"Written to the file";
            match file.write_all(content) {
                Err(why) => panic!("couldn't write to file at {}: {}", display,
                                                           why.description()),
                Ok(_) => println!("Written with content - {}", str::from_utf8(content).unwrap()),
            }
        },
        OpenFileMode::Append => {
            let content = b"\nAppended to the file";
            match file.write_all(content) {
                Err(why) => panic!("couldn't append to file at {}: {}", display,
                                                            why.description()),
                Ok(_) => println!("Appended with content - {}", str::from_utf8(content).unwrap()),
            }
            
        },
        _ => panic!("This filemode operation isn't supported")
    };
}

fn main() {
    let inputs: Vec<String> = env::args().collect();
    if inputs.len() < 3 {
        panic!("\n    Usage: ./fileio <filepath> <operation>".to_owned() + 
              &"\n      filepath -> relative or absolute path of file" + 
              &"\n      operation -> Create/Write/Read/Append");
    }
    let path = Path::new(&inputs[1]);
    let filemode: OpenFileMode = inputs[2].parse().unwrap();

    operate_on_file(&path, &filemode);
}