//! Create the Error, ErrorKind, ResultExt, and Result types.
use ezo_common;

error_chain! {
    errors {
        // The unsuccessful response code
        I2CRead {
            description ("unsuccessful device read")
            display ("response was not obtainable")
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
