//! Create the Error, ErrorKind, ResultExt, and Result types.
use ezo_common;

error_chain! {
    errors {
        // The response could not be parsed
        ResponseParse {
            description ("could not parse response")
        }
    }
    links {
        Ezo(ezo_common::errors::Error, ezo_common::errors::ErrorKind);
    }
}
