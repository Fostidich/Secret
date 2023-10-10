use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::Read;
use crate::util::err_codes::{IO_ERROR, SERDE_ERROR};
use crate::util::exiting::Catch;
use crate::util::file;

/// Giving the path to a json file, and a type to deserialize data into,
/// this function returns json data as the specified type.
///
/// # Errors
///
/// Execution ends if problems occurs while opening the file,
/// or if the retrieved string is not deserializable in the provided type.
///
/// # Example
///
/// ```usage_with_type
/// let list: Vec<Entry> = get_from_json::<Vec<Entry>>(LIST_PATH);
///     if list.is_empty() {
///         println!("No entries to show.");
///         return;
///     }
///     for entry in list {
///         println!("{}", entry);
///     }
/// ```
///
/// (See also [file::open_file])
pub fn get_from_json<T>(path: &str) -> T where T: DeserializeOwned {
    let mut file: File = file::open_file(path);
    let mut buff: String = String::new();
    file.read_to_string(&mut buff).catch(IO_ERROR);
    serde_json::from_str::<T>(&buff).catch(SERDE_ERROR)
}
