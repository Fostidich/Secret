use std::process::exit;

/// Error code represent a possible error by assigning a number and a message to be written on stderr.
/// Many codes are stored as public constants in the error codes file.
pub struct ErrorCode {
    pub(super) name: &'static str,
    pub(super) code: i32,
    pub(super) message: &'static str
}

impl ErrorCode {
    /// Print function for [ErrorCode] outputs on stderr the formatted fields.
    ///
    /// # Example
    ///
    /// ```print_example
    /// let err: ErrorCode = UNKNOWN_ERR;
    /// err.print();
    /// // will output:
    /// // [ERROR] 1-UNKNOWN_ERROR: unknown error!
    /// ```
    fn print(&self) {
        eprintln!("[ERROR] {}-{}: {}!", self.code, self.name, self.message)
    }
}

pub trait Catch<T, E> {
    /// Catch function is used to unwrap a [Result] and return its [Ok] content.
    /// If an [Err] is found, program is exited with the error code provided and
    /// respective message printed on stderr.
    ///
    /// # Example
    ///
    /// ```catch_error
    /// let test: Result<i32, i32> = Err(1);
    /// test.catch(UNKNOWN_ERROR);
    /// ```
    fn catch(self, err: ErrorCode) -> T;
}

impl<T, E> Catch<T, E> for Result<T, E> {
    fn catch(self, err: ErrorCode) -> T {
        match self {
            Err(_) => {
                err.print();
                exit(err.code);
            }
            Ok(d) => d
        }
    }
}

/// End function is used to immediately end the execution.
/// Program is exited with the error code provided and respective message printed on stderr.
///
/// # Example
///
/// ```end_error
/// if args.len() == 1 {
///     end(UNKNOWN_ERROR)
/// }
/// ```
pub fn end(err: ErrorCode) {
    err.print();
    exit(err.code);
}