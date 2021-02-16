use std::time::SystemTime;

use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Company {
    name: String,
    created_at: SystemTime,
    updated_at: SystemTime,
    version: u64,
    is_cpo: bool,
    is_emp: bool,
}
