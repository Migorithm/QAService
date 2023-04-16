use std::{collections::HashMap, ops::Deref};

use handle_errors::Error;

#[derive(Debug)]
pub struct Pagination {
    pub limit: Option<i32>,
    pub offset: i32,
}

/// # Example Query
/// `/questions?limit=1&offset=10`
/// # Example usage
/// ```
/// let mut query = HashMap::new();
/// query.insert("limit".to_string(),"1".to_string());
/// query.insert("offset".to_string(),"10".to_string());
/// let p = types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.limit,1);
/// assert_eq!(p.offset,10);
/// ```
pub fn extract_pagination(params: HashMap<String, i32>) -> Result<Pagination, Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        Ok(Pagination {
            limit: Some(*params.get("limit").unwrap()),
            offset: *params.get("offset").unwrap(),
        })
    } else {
        Err(Error::Missing)
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: None,
            offset: 0,
        }
    }
}
