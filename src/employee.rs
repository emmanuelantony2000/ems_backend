use time::Date;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Employee {
    name: String,
    email: String,
    phone_number: String,
    dob: Date,
    role: String,
    experience: u32,
    address: Address,
}

struct Address {
    city: Option<String>,
    district: Option<String>,
    state: Option<String>
}
