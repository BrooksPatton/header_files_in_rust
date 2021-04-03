use super::hello_h::Hello;

impl Hello {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }

    /// Speak will say hello using the stored name
    ///
    /// ```
    /// use header_files_in_rust::hello::hello_h::Hello;
    ///
    /// let hello = Hello::new("Brookzerker");
    /// assert_eq!(hello.speak(), "Hello Brookzerker!");
    /// ```
    pub fn speak(&self) -> String {
        format!("Hello {}!", &self.name)
    }
}
