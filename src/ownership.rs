#[allow(dead_code)]
pub fn vars() {
    // All primitive datatypes and their copies are pushed onto the stack.
    // This is because of their known size at compile time, unlike growable
    // vars like String, that must be allocated on heap.
    let s = "hello"; // allocated on stack
    let z = s;
    let x = 5; // allocated on stack
    let y = x; // also allocated on stack.
    log::info!("s : {s}, x : {x}, y: {y}, z {z}");

    // vars of growable/unknown size are allocated on heap
    let b = String::from(s);

    log::info!("{s}");

    // this will move the mem ref on b to c and invalidate b for any further use.
    let c = b;
    log::info!("c {c}");

    // below will error during compilation.
    // b is stored on heap initially
    // b is assigned to c. At this point ref to b on heap is moved
    // b is no longer available now
    //log::info!("b {b}");
    drop(c); // Rust will call drop automatically when the scope is over
}

#[allow(dead_code)]
pub fn func_with_return(){
     // fn_gives_ownership moves the ownership to s1
     let s1 = fn_gives_ownership();
     log::info!("s1 : {s1}");
 
     // fn_takes_and_gives_ownership takes the ownership of s1
     // and returns new String and moves the ownership to s2
     let s2 = fn_takes_and_gives_ownership(s1);
     log::info!("s2 : {s2}");
     //log::info!("s1 : {s1}");
}

#[allow(dead_code)]
fn fn_takes_and_gives_ownership(s: String) -> String {
    // takes ownership of s1
    s // works on s1, returns and moves the ownership to caller
}

#[allow(dead_code)]
fn fn_gives_ownership() -> String {
    String::from("hello")
}

#[allow(dead_code)]
pub fn func_without_return(){
    let s = String::from("value"); // allocated on heap

    log::info!("Before calling take_string_ownership. s -> {s}"); // s valid here
    take_string_ownership(s); // reference moved to take_string_ownership

    //log::info!("After calling take_string_ownership. s -> {s}"); // s invalid here

    // x is primitive type. its copied onto the stack. nothing special
    let x = 5;
    log::info!("Before calling take_int_copy. x -> {x}"); // x valid here
    take_int_copy(x);
    log::info!("After calling take_int_copy. x -> {x}"); // x valid here
}


#[allow(dead_code)]
fn take_int_copy(i: i32) {
    // creates copy
    log::info!("inside take_int_copy. i-> {i}")
} // i goes out of scope.

#[allow(dead_code)]
fn take_string_ownership(s: String) {
    log::info!("inside take_string_ownership. s-> {s}")
} // s goes out of scope. Rust calls drop(s) and memory is freed

