use crate::enums::ColTypesEnum;

//Add structs and their implementations to each of the created files
pub fn get_col_type(col_type: &str) -> ColTypesEnum {
    match col_type {
        "VARCHAR" => ColTypesEnum::VARCHAR,
        "INTEGER" => ColTypesEnum::INTEGER,
        "FLOAT" => ColTypesEnum::FLOAT,
        "UUID" => ColTypesEnum::UUID,
        "BOOLEAN" => ColTypesEnum::BOOLEAN,
        _ => ColTypesEnum::VARCHAR,
    }
}