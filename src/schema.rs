use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoQuery {
    pub uid: Option<String>,
    #[serde(rename = "upperDate")]
    pub upper_date: Option<String>,
    #[serde(rename = "lowerDate")]
    pub lower_date: Option<String>,

    // Filter Options:
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchQuery {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
