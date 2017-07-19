//! Create the Error, ErrorKind, ResultExt, and Result types.
use ezo_common;

error_chain! {
    errors {
        // The unsuccessful response code
        I2CRead {
            description ("unsuccessful device read")
            display ("response was not obtainable")
        }
        // The response is not nul-terminated, or it is not valid ASCII/UTF-8
        MalformedResponse {
            description ("malformed response")
            display ("response is not a valid nul-terminated UTF-8 string")
        }
        // The response could not be parsed
        ResponseParse {
            description ("could not parse response")
        }
        // Any response code that is not `Success`
        UnsuccessfulResponse {
            description ("unsuccessful response code")
            display ("response code was not successful")
        }
    }
    links {
        Ezo(ezo_common::errors::Error, ezo_common::errors::ErrorKind);
    }
}
