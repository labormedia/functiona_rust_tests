use std::fs;

pub trait UnitTest {
    type DataElement;
    // type ReturnType;
}

pub fn apply<'a,T: 'a,U: 'a, W: 'a>(data_filepath: &'a str, instance: impl FnOnce(T) -> U, parser: impl FnOnce(&str) -> Box<T>) -> T 
where  
    T: 'a,
    U: 'a 
{
    let raw_data = fs::read_to_string(data_filepath)
        .expect("No input");
    
    let data: Box<T> = parser(&raw_data);

    // instance(data);
    *data
}