
#[cfg(test)] 
mod testes {
    use std::{fs::File, io::{Read, Write}, path::Path};

    use json_parser::save;

    #[test]
    fn basic_test() {
        let path = Path::new("./tests/foo.txt");

        let initial_data: &str = "Hello, world!";
        let mut file: File = std::fs::OpenOptions::new().write(true)
                                                        .read(true)
                                                        .create(true)
                                                        .truncate(true)
                                                        .open(path)
                                                        .unwrap();
        file.write_all(initial_data.as_bytes()).unwrap();

        let foo = 12345;
        save(path, foo).unwrap();

        let mut read: String = String::new();
        file.read_to_string(&mut read).unwrap();

        assert_ne!(initial_data.to_string(), read);
    }
}



