use std::fmt::{Debug, Formatter};

pub enum GBizApiError {
    Connection(String),
    JsonParse(String),
}

impl GBizApiError {
    pub fn to_string(&self) -> String {
        match self {
            GBizApiError::Connection(e) => { e.to_string() }
            GBizApiError::JsonParse(e) => { e.to_string() }
        }
    }
}

impl Debug for GBizApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}
