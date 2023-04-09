use std::collections::HashMap;

use handle_errors::Error;

pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

/// # Example Query
/// `/questions?start=1&ebd=10`
/// # Example usage
/// ```
/// let mut query = HashMap::new();
/// query.insert("start".to_string(),"1".to_string());
/// query.insert("end".to_string(),"10".to_string());
/// let p = types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.start,1);
/// assert_eq!(p.end,10);
/// ```
pub fn extract_pagination(params: HashMap<String, i32>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        Ok(Pagination {
            start: *params.get("start").unwrap() as usize,
            end: *params.get("end").unwrap() as usize,
        })
    } else {
        Err(Error::Missing)
    }
}
