fn main() {
    println!("Hello, world!");

    let value = Some(6);
    if let Some(x) = value {
        println!("Value is {}", x);
    }
    else {
        println!("No value")
    }

    use my_mod::MyTrait;
    let obj = my_mod::create_obj();
    println!("Value is {}", obj.do_stuff());
}

mod my_mod {
    pub trait MyTrait {
        fn do_stuff(&self) -> u32;
    }

    struct MyPrivateType {
    }

    impl MyTrait for MyPrivateType {
        fn do_stuff(&self) -> u32 {
            return 42;
        }
    }

    pub fn create_obj() -> impl MyTrait {
        MyPrivateType{}
    }
}