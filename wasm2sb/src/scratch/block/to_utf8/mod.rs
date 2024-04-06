pub mod check_unicode;
pub mod check_uppercase;
pub mod generator;
pub mod unicode;

pub const PRE_UNICODE: &str = "to_utf8_";

pub fn tmp_list_name() -> String {
    format!("{PRE_UNICODE}tmp")
}

pub fn upper_case_data_list_name() -> String {
    format!("{PRE_UNICODE}uppercase_data")
}
