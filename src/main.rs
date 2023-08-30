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
    print_type_of(&obj);
    so_action(obj);

    let obj2 = my_mod::MyOtherType{};
    so_action(obj2);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn so_action(obj : impl my_mod::MyTrait) {
    println!("Value is {} in so_action", obj.do_stuff());

    fn f() {}
    print_type_of(&f);
}


fn test_dyn(obj : &dyn my_mod::MyTrait) {

}
mod my_mod {
    pub trait MyTrait {
        fn do_stuff(&self) -> u32;
    }

    struct MyPrivateType;

    pub struct MyOtherType;

    impl MyTrait for MyPrivateType {
        fn do_stuff(&self) -> u32 {
            42
        }
    }

    impl MyTrait for MyOtherType {
        fn do_stuff(&self) -> u32 {
            100
        }
    }

    pub fn create_obj() -> impl MyTrait {
        MyPrivateType{}
    }

    pub fn create_obj_2(b: bool) -> Box<dyn MyTrait> {
        if b {
            return Box::new(MyPrivateType{});
        }
        else {
            return Box::new(MyOtherType{});
        }
    }
}