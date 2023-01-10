use std::fs;

pub fn apply<'a,T: 'a,U: 'a>(data_filepath: &'a str, instance: impl Fn(&T) -> U, parser: impl Fn(&String) -> T) -> U
where  
    T: 'a + std::convert::From<T>,
    U: 'a 
{
    let raw_data = fs::read_to_string(data_filepath)
        .expect("Input error");
    
    let collection: T = parser(&raw_data).into();
    instance(&collection)
}