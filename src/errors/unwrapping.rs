use std::process::exit;

//TODO: manage error messages

const UNKNOWN_ERROR: u8 = 1;

pub trait Catch<T, E> {
    /// Catch function is used to unwrap a [Result] and return its [Ok] content.
    /// If an [Err] is found, program is exited with the error code provided and
    /// respective message printed on stderr.
    ///
    /// # Example
    ///
    /// ```possible_error
    /// let mut current_dir = env::current_exe().catch(10);
    /// ```
    fn catch(self, msg: &str) -> T;
}

impl<T, E> Catch<T, E> for Result<T, E> {
    fn catch(self, msg: &str) -> T {
        match self {
            Err(_) => {
                eprintln!("{}", msg);
                exit(1)
            }
            Ok(d) => d
        }
    }
}