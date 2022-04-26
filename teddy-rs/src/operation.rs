use strum_macros::EnumString;

#[derive(Debug, EnumString)]
pub enum Operation {
    GET,
    SET,
    DELETE,
    EXISTS,
}
