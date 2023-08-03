/*
   1. At any given time, you can have either one mutable reference or any number of immutable references.
   2. References must always be valid.
*/

#[allow(dead_code)]
pub fn run() {
    // on heap : local scope
    let x = String::from("super");
    log::info!("x is {}", x);

    // &x -> read only ref to x that can be borrowed, without owning
    let length = get_string_len(&x); // ref borrowed by called function
    log::info!("x is {x}, length {length}");

    // mutable String
    let mut s = String::from(x);
    log::info!("s is {s}");

    mutate_string(&mut s);
    log::info!("mutated string is {s}");
}

// this function borrows ref to String.
// the borrowed ref can be only used for read only operations.
// no mutable operations can be performed on borrowed refs
fn get_string_len(item: &String) -> usize {
    item.len()
}

// mutate_string borrows a mutable string ref
fn mutate_string(item: &mut String) {
    item.push_str(" ⭐️");
}

#[allow(dead_code)]
pub fn mutable_ref() {
    let mut ss = String::from("something");
    let y = &mut ss;
    log::info!("y is {y}"); // mutable borrow used here

    // another borrow can happen, as long as its not simultaneously being used togther with y
    // which can cause data race
    let z = &mut ss;
    log::info!("z is {z}");

    // below will error out, because simultaneous use of y,z can cause data race
    // log::info!("y -> {y} , z -> {z}")
}

/*
   1. a var can have multiple immutable ref
   2. If a var has immutable ref, it can not have mutable ref
*/
#[allow(dead_code)]
pub fn combine_mut_unmut_ref() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    // creating a mutable ref when immutable ref exists
    //let r3 = &mut s; // BIG PROBLEM

    log::info!("r1 {}", r1);
    log::info!("r2 {}", r2);
}
