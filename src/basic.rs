use std::io::stdin;

#[allow(dead_code)]
pub fn run_conditionals() {
    // assign using if else
    let condition = true;
    let x = if condition { 5 } else { 6 };
    log::info!("x is {x}");

    // loop returning values
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    log::info!("The result is {result}");

    // nested loops + loop lables
    let mut count = 0;
    'counting_up: loop {
        log::info!("counting_up = {count}");
        let mut remaining = 10;
        loop {
            log::info!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    log::info!("End count = {count}");
}

#[allow(dead_code)]
pub fn run_functions() {
    let num = 5;
    num_print(num);

    num_increment(num);
    log::info!("number after increment {num}");
    log::info!("number after increment {}", num_increment_and_return(num));
}

#[allow(dead_code)]
fn num_print(x: i32) {
    log::info!("number is {x}")
}
fn num_increment(mut x: i32) {
    log::info!("before increment {x}");
    x += 1;
    log::info!("after increment {x}")
}
fn num_increment_and_return(mut x: i32) -> i32 {
    log::info!("before increment {x}");
    x += 1;
    log::info!("after increment {x}");
    return x;
}

#[allow(dead_code)]
pub fn invalid_array_access() {
    let source_array = [42; 5];
    for (k, v) in source_array.iter().enumerate() {
        log::info!("item {k} is {v}")
    }

    loop {
        log::info!("Please enter an index");
        let mut line = String::new();
        stdin().read_line(&mut line).expect("could not read line");

        let index: usize = match line.trim_end().parse() {
            Ok(idx) => idx,
            Err(err) => {
                log::error!("invalid input {:?}", err);
                continue;
            }
        };

        if index > (source_array.len() - 1) {
            log::error!("invalid array index");
            continue;
        }

        log::info!("your element is {}", source_array[index]);
        break;
    }
}

#[allow(dead_code)]
pub fn run_variables() {
    // const
    const THREE_HRS_IN_SECS: u32 = 60 * 60 * 3;
    log::info!("THREE_HRS_IN_SECS is {THREE_HRS_IN_SECS}");

    // immute / mut vars
    let mut x = String::from("prakhar");
    log::info!("x is {x}");
    x.push_str(" some more text");
    log::info!("x is {x}");

    let x = 5;
    log::info!("x is {x}");

    // shadow
    {
        log::info!("shadow var before {x}");
        let x = x * 2;
        log::info!("shadow var after {x}");
    }
    log::info!("x outside scope is {x}");

    let tup = (23, String::from("value"), 42.00);
    log::info!("tup.0 {}", tup.0);
    log::info!("tup.1 {}", tup.1);
    log::info!("tup.2 {}", tup.2);

    // immutable arrays
    let arr = [3; 5];
    log::info!("arr {:?}", arr);
    for element in arr.iter() {
        log::info!("element {:?}", element);
    }

    // will error out, if we try to change the value of any element
    // arr[0] = 0
}

