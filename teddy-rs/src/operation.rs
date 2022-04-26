///! In this file we define all possible operations supported by Teddy
use strum_macros::EnumString;

/// This is an enum containing all supported operations
#[derive(Debug, EnumString)]
pub enum Operation {
    GET,
    SET,
    DELETE,
    EXISTS,
}
