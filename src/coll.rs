pub fn run() {
    let mut v: Vec<i32> = Vec::new();

    let range = [1, 2, 3, 4, 5];
    for i in range {
        v.push(i)
    }

    match v.get(3) {
        Some(e) => {
            log::info!("the 3rd val is {e}");
        }
        None => log::error!("value does not exists")
    }

    let x = &mut v[3];
    *x = 34;

    match v.get(3) {
        Some(e) => {
            log::info!("the 3rd val is {e}");
        }
        None => log::error!("value does not exists")
    }


    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");
    let hello = String::from("नमस्ते");

    for c in  hello.chars(){
        println!("{c}");
    }
}