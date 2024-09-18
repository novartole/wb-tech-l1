use std::borrow::Cow;

pub trait Action {
    fn say(&self);
}

pub struct Person<'a> {
    name: Cow<'a, str>,
}

impl Action for Person<'_> {
    fn say(&self) {
        println!("Hello, {}", self.name);
    }
}

fn main() {
    // name is static str
    Person {
        name: Cow::Borrowed("static world"),
    }
    .say();

    // name is str
    let local_name = &"local world".to_owned();
    Person {
        name: Cow::Borrowed(local_name),
    }
    .say();

    // name string
    Person {
        name: Cow::Owned("owned world".to_owned()),
    }
    .say();
}
