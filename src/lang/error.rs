pub mod parser;

pub trait Reportable {
    /// Prints the error to stderr
    fn report(&self, src_id: &str, src: &str);
}
