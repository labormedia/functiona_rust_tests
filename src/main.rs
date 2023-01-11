#[derive(Debug)]
struct MyStruct {
    my_string: String,
    my_uint32: u32
}

impl MyStruct {
    fn my_associated_function(&mut self, my_string: &str, my_uint32: u32) -> &Self {
        self.my_string = my_string.into();
        self.my_uint32 = my_uint32;
        self
    }

    fn bind(&mut self, my_function: fn(&mut MyStruct) -> &Self) -> &Self {
        my_function(self);
        self
    }

    fn curry(&'static mut self, my_function: fn(&mut MyStruct) -> &Self, my_struct: &mut MyStruct) -> impl FnOnce() -> &'static MyStruct {
        self.my_string = self.my_string.to_owned() + &my_struct.my_string;
        (|my_self| move || { my_function(my_self) })(self)
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tests() {
        let an_instance= MyStruct {
            my_string: "World".into(),
            my_uint32: 42,
        };

        // Making something 'static by intentionally leaking memory
        //
        // Step 1: put the thing you don't want to drop (free the memory for)
        // inside a box.
        let my_new_instance = Box::new(an_instance);

        // Step 2: leak the box.
        // This will cause an intentional memory leak
        let my_new_instance = Box::leak(my_new_instance);

        println!("Check A: {:#?}", my_new_instance);

        println!("Check B: {:#?}", my_new_instance.my_associated_function("Hello", 65));

        fn lets_create_a_function<'a> (myself: &'a mut MyStruct) -> &'a MyStruct {
            myself.my_string = myself.my_string.to_owned() + " Tomato".into();
            myself.my_uint32 = myself.my_uint32 + 78;
            myself
        }
    
        fn lets_create_a_second_function(myself: &mut MyStruct) -> &MyStruct {
            myself
        }
    
        my_new_instance.bind(lets_create_a_function);
    
        println!("Check C: {:#?}", my_new_instance);
    
        let my_curry = my_new_instance.curry(lets_create_a_second_function, &mut MyStruct { my_string: " Pineapple".into(), my_uint32: 0 });
    
        println!("Check D: {:#?}", my_curry());

        // let some_other_instance = Box::new(an_instance);

        // let some_other_instance = Box::leak(some_other_instance);

        // let my_curry = some_other_instance.curry(lets_create_a_function, &mut MyStruct { my_string: " Pineapple".into(), my_uint32: 0 });

        // println!("Check E: {:#?}", my_curry())s
    }

 
}