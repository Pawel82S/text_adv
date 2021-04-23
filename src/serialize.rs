use std::{
    fs::File,
    io::{self, BufReader, BufWriter},
};

pub trait Serialize<T = Self> {
    fn load_from_file(file_name: &str) -> io::Result<T>;
    fn save_to_file(&self, file_name: &str) -> io::Result<()>;
    fn load_from_reader(reader: &mut BufReader<File>) -> Result<T, &str>;
    fn save_to_writer(&self, writer: &mut BufWriter<File>);
}
