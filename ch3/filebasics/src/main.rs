//!Simulating files onew step at a time
/// Represents a "file",
/// which probably lives on a file system.
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    /// New files are assumed to be empty, but a name is required.
    /// Creates a new, empty `File`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let f = File::new("f1.txt");
    /// ```
    pub fn new(name: &str)-> Self {
        Self {
            name: String::from(name),
            data: Vec::new(),
        }
    }
    /// Returns the file's length in bytes
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Returns the file's name.
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
fn main() {
    let f1 = File::new("f1.txt");
    let f1_length = f1.len();
    let f1_name = f1.name();
    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}
