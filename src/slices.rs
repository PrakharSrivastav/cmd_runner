#[allow(dead_code)]
pub fn run() {
    let input = String::from("hello world");
    log::info!("first word in '{}' is '{}'",input,first_word(&input));
}

#[allow(dead_code)]
fn first_word(input: &str) -> &str {
    let bytes = input.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &input[..i];
        }
    }

    input
}

// an array should always be declared along with size
#[allow(dead_code)]
pub fn array() {

    // array declaration and assignment
    let a1: [i32; 5] = [1, 2, 3, 4, 5];
    log::info!("a1 => {:?}, len {}",a1,a1.len());

    // assign all elements to same value
    let a2 = [3; 5];
    log::info!("a2 => {:?}, len {}",a2,a2.len());

    let a1s = &a1;
    log::info!("borrow the whole array as slice {:?}",a1s);

    let a1s = &a1[0..3];
    log::info!("borrow subset of array as slice {:?}",a1s);

    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    log::info!("empty_array {:?}",empty_array);
}