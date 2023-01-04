#[derive(Debug)]
struct MyStruct {
    my_string: String,
    my_int32: u32
}

impl MyStruct {
    fn my_associated_function(&mut self, my_string: &str, my_uint32: u32) -> &Self {
        self.my_string = my_string.into();
        self.my_int32 = my_uint32;
        self
    }

    fn bind(&mut self, my_function: fn(&mut MyStruct) -> &Self) -> &Self {
        my_function(self);
        self
    }
}

fn main() {
    let mut my_new_instance = MyStruct {
        my_string: "World".into(),
        my_int32: 42,
    };
    println!("Check A: {:#?}", my_new_instance);
    println!("Check B: {:#?}", my_new_instance.my_associated_function("Hello", 65));

    fn lets_create_a_function(myself: &mut MyStruct) -> &MyStruct {
        myself.my_string = myself.my_string.to_owned() + " Tomato".into();
        myself.my_int32 = myself.my_int32 + 78;
        myself
    }

    my_new_instance.bind(lets_create_a_function);

    println!("Check C: {:#?}", my_new_instance);
}
