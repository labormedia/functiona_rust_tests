use std::fs;

pub fn apply<T,U>(data_filepath: String, instance: impl FnOnce(&T) -> U, parser: impl FnOnce(&String) -> &T) {
    let raw_data = fs::read_to_string(data_filepath)
        .expect("No input");
    
    let data: &T = parser(&raw_data);

    instance(data);
}