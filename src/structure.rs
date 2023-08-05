struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

#[allow(dead_code)]
pub fn basic() {
    let u = User {
        name: String::from("Prakhar"),
        email: String::from("test@prakhar.com"),
        active: true,
        sign_in_count: 3,
    };

    log::info!("u.name is {}",u.name);
    log::info!("u.email is {}",u.email);
    log::info!("u.active is {}",u.active);

    // create a mutable user
    log::info!("u.sign_in_count is {}",u.sign_in_count);

    let mut u2 = User { ..u };
    log::info!("u2.sign_in_count is {}",u2.name);
    u2.name = String::from("Prakhar Srivastav");
    log::info!("u2.sign_in_count is {}",u2.name);

    // create a new user
    let chelsea = create_user(String::from("Chelsea"), String::from("chelsea@test.com"));
    log::info!("chelsea.name is {:?}",chelsea.name);

    let color = Color(0, 255, 0);
    let point = Point(0, 0, 0);

    log::info!("color.1 is {:?}",color.1);
    log::info!("point.1 is {:?}",point.1);
}

#[allow(dead_code)]
fn create_user(name: String, email: String) -> User {
    User {
        name,      // shorthand assign
        email,     // shorthand assign
        active: false,
        sign_in_count: 0,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    length: i32,
}

impl Rectangle {
    // not a method. like a static method in java
    fn square(size: i32) -> Self {
        Self { length: size, width: size }
    }

    // a method.
    fn area(&self) -> i32 {
        self.width * self.length
    }

    #[allow(dead_code)]
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.length >= other.length
    }
}

fn calculate_area(r: &Rectangle) -> i32 {
    return r.width * r.length;
}

    #[allow(dead_code)]
pub fn area_rectangle() {
    let r1 = Rectangle { length: 10, width: 10 };

    log::info!("area of {:?} is {}",r1, calculate_area(&r1));
    log::info!("area of {:?} is {}",r1, r1.area());
    log::info!("square is {:?}", Rectangle::square(10));
}