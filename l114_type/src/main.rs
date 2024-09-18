use std::{
    any::{self, Any},
    borrow::Cow,
    cell::RefCell,
    fmt::Debug,
    rc::Rc,
};

#[derive(Debug)]
struct CustomType {
    _field: *const (),
}

fn main() {
    let values: [&dyn Any; 9] = [
        // privitives
        &(),
        &false,
        &'1',
        &2u8,
        &3i8,
        &"4",
        // naitive complex type
        &5.to_string(),
        // constructed complex type
        &Rc::new(RefCell::new(Cow::Borrowed("6"))),
        // custom type
        &CustomType {
            _field: &() as *const _,
        },
    ];

    for value in values {
        print_value_and_type(value);
    }
}

fn print_value_and_type(value: &dyn Any) {
    if let Some(val) = value.downcast_ref::<()>() {
        print_value_and_type(val);
    } else if let Some(bool) = value.downcast_ref::<bool>() {
        print_value_and_type(bool);
    } else if let Some(char) = value.downcast_ref::<char>() {
        print_value_and_type(char);
    } else if let Some(u8) = value.downcast_ref::<u8>() {
        print_value_and_type(u8);
    } else if let Some(i8) = value.downcast_ref::<i8>() {
        print_value_and_type(i8);
    } else if let Some(str) = value.downcast_ref::<&str>() {
        print_value_and_type(str);
    } else if let Some(string) = value.downcast_ref::<String>() {
        print_value_and_type(string);
    } else if let Some(rc_refcell_cow) = value.downcast_ref::<Rc<RefCell<Cow<'_, str>>>>() {
        print_value_and_type(rc_refcell_cow);
    } else if let Some(custom_type) = value.downcast_ref::<CustomType>() {
        print_value_and_type(custom_type);
    } else {
        unimplemented!()
    }

    fn print_value_and_type<T>(val: &T)
    where
        T: Debug,
    {
        println!("{:?}: {}", val, any::type_name::<T>());
    }
}
