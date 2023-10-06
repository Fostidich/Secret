#![allow(dead_code)]

use crate::errors::exiting::ErrorCode;

/// Error is to be used just in exceptional cases where no other error code can describe the error.
pub const UNKNOWN_ERROR: ErrorCode = ErrorCode {
    name: "UNKNOWN_ERROR",
    code: 1,
    message: "unknown error"
};

/// Error is used if no command is provided while running the program.
pub const NO_COMMAND_PROVIDED: ErrorCode = ErrorCode {
    name: "NO_COMMAND_PROVIDED",
    code: 2,
    message: "no command provided! Try using \"scrt help\""
};

/// Error is used if arguments are invalid, or in a wrong quantity.
pub const INVALID_ARGUMENTS: ErrorCode = ErrorCode {
    name: "INVALID_ARGUMENTS",
    code: 3,
    message: "invalid arguments! Try using \"scrt help\""
};

/// Error is used if provided command is unknown.
pub const UNKNOWN_COMMAND: ErrorCode = ErrorCode {
    name: "UNKNOWN_COMMAND",
    code: 4,
    message: "unknown command! Try using \"scrt help\""
};

/// Error is used if the directory of the executable is not found.
pub const DIR_NOT_FOUND: ErrorCode = ErrorCode {
    name: "DIR_NOT_FOUND",
    code: 5,
    message: "failed to get current directory"
};

/// Error is used if program is unable to open, create or seek a file, or the folders of its path.
pub const FILE_FAILURE: ErrorCode = ErrorCode {
    name: "FILE_FAILURE",
    code: 6,
    message: "failed to open or create a file, or its directory"
};

/// Error is used in case of failure while retrieving metadata from a file.
pub const NO_DATA_FOUND: ErrorCode = ErrorCode {
    name: "NO_DATA_FOUND",
    code: 7,
    message: "failed to retrieve metadata"
};

/// Error is used in case of failure while reading or writing a file.
pub const IO_ERROR: ErrorCode = ErrorCode {
    name: "IO_ERROR",
    code: 8,
    message: "failed to read or write file"
};

/// Error is used if unable to serialize/deserialize data.
pub const SERDE_ERROR: ErrorCode = ErrorCode {
    name: "SERDE_ERROR",
    code: 9,
    message: "failed to serialize/deserialize data"
};