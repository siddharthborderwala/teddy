use std::str::FromStr;

use crate::operation::Operation;
use crate::value::Value;

pub type QueryResult = Result<Value, String>;

#[derive(Debug)]
pub struct Query<'a> {
    pub original_query_string: &'a str,
    pub operator: Operation,
    pub operand: &'a str,
    pub values: Option<&'a str>,
}

impl<'a> Query<'a> {
    pub fn new(qs: &'a str) -> Result<Self, String> {
        let vector = qs
            .trim()
            .splitn(3, ' ')
            .map(|s| s.trim())
            .collect::<Vec<&'a str>>();
        let size = vector.len();

        if size == 0 {
            return Err("Missing operator".into());
        }
        let operator =
            Operation::from_str(vector.get(0).unwrap()).map_err(|_| "Unknown operation")?;

        if size == 1 {
            return Err("Missing key".into());
        }

        Ok(Self {
            original_query_string: qs,
            operator: operator,
            operand: vector.get(1).unwrap(),
            values: vector.get(2).cloned(),
        })
    }
}
