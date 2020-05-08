extern crate rand;

use rand::Rng;
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
    Remove,
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
            _ => panic!("Invalid Filemode '{}' provided", &s),
        }
    }
}

fn operate_on_file(path: &Path, filemode: &OpenFileMode) {
    if mem::discriminant(filemode) == mem::discriminant(&OpenFileMode::Remove) {
        match fs::remove_file(path) {
            Err(why) => panic!(
                "couldn't remove the file at path {}: {}",
                path.display(),
                why.description()
            ),
            Ok(_) => println!("Removed the file"),
        };
        return;
    }
    let mut open_options = fs::OpenOptions::new();
    match filemode {
        OpenFileMode::Read => open_options.read(true),
        OpenFileMode::Write => open_options.write(true),
        OpenFileMode::Append => open_options.append(true),
        OpenFileMode::Create => open_options.write(true).create_new(true),
        _ => panic!("This filemode operation isn't supported"),
    };

    let display = path.display();
    let mut file = match open_options.open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!(
            "couldn't perform the file operation {}: {}",
            display,
            why.description()
        ),
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
                Err(why) => panic!("couldn't read file at {}: {}", display, why.description()),
                Ok(_) => println!("Read the content - {}", &buf),
            }
        }
        OpenFileMode::Write => {
            let content = b"Written to the file";
            match file.write_all(content) {
                Err(why) => panic!(
                    "couldn't write to file at {}: {}",
                    display,
                    why.description()
                ),
                Ok(_) => println!(
                    "Written with content - {}",
                    str::from_utf8(content).unwrap()
                ),
            }
        }
        OpenFileMode::Append => {
            let content = b"\nAppended to the file";
            match file.write_all(content) {
                Err(why) => panic!(
                    "couldn't append to file at {}: {}",
                    display,
                    why.description()
                ),
                Ok(_) => println!(
                    "Appended with content - {}",
                    str::from_utf8(content).unwrap()
                ),
            }
        }
        _ => panic!("This filemode operation isn't supported"),
    };
}

pub fn main() {
    let inputs: Vec<String> = env::args().collect();
    if inputs.len() < 3 {
        panic!(
            "\n    Usage: ./fileio <filepath> <operation>".to_owned()
                + &"\n      filepath -> relative or absolute path of file"
                + &"\n      operation -> Create/Write/Read/Append"
        );
    }
    let path = Path::new(&inputs[1]);
    let filemode: OpenFileMode = inputs[2].parse().unwrap();

    operate_on_file(&path, &filemode);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_remove() {
        let mut rng = rand::thread_rng();
        let filepath_str = format!("/tmp/abc-fileio-{}.txt", rng.gen_range(0, 1000));
        let filepath = Path::new(&(filepath_str));
        assert_eq!((), operate_on_file(filepath, &OpenFileMode::Create));
        assert_eq!((), operate_on_file(filepath, &OpenFileMode::Remove));
    }

    #[test]
    #[should_panic]
    fn test_create_panic_no_access() {
        let filepath = Path::new(&"/etc/abc-fileio.txt");
        operate_on_file(filepath, &OpenFileMode::Create);
        operate_on_file(filepath, &OpenFileMode::Remove);
    }

    #[test]
    #[should_panic]
    fn test_create_panic_invalid_path() {
        let filepath = Path::new(&"/tmp/abc/def/abc-fileio.txt");
        operate_on_file(filepath, &OpenFileMode::Create);
        operate_on_file(filepath, &OpenFileMode::Remove);
    }

    #[test]
    fn test_write_and_read() {
        let mut rng = rand::thread_rng();
        let filepath_str = format!("/tmp/abc-fileio-{}.txt", rng.gen_range(0, 1000));
        let filepath = Path::new(&filepath_str);
        assert_eq!((), operate_on_file(filepath, &OpenFileMode::Create));
        assert_eq!((), operate_on_file(filepath, &OpenFileMode::Write));
        let mut file = fs::File::open(&filepath_str).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "Written to the file");
        assert_eq!((), operate_on_file(filepath, &OpenFileMode::Remove));
    }

    #[test]
    fn test_append_and_read() {
        let filepath = Path::new(&"/tmp/abc-fileio.txt");
        assert_eq!((), operate_on_file(filepath, &OpenFileMode::Create));
        assert_eq!((), operate_on_file(filepath, &OpenFileMode::Write));
        assert_eq!((), operate_on_file(filepath, &OpenFileMode::Append));
        assert_eq!((), operate_on_file(filepath, &OpenFileMode::Remove));
    }
}
