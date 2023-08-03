# cmd_runner

run shell commands provided in config.yaml file. Currently supports only bash scripts

# running
```shell
# build
cargo build

# run
cargo run

# release -> generates optimized executable binary
cargo build --release
```


## rust ownership
- all data store on stack should have fixed size. Data with unknown size at compile time must be stored on heap.
- the data on the heap is always accessed via pointers.
- but pointers themselves have fixed size and can be stored on stack.
- storing on stack is cheap, allocating on heap requires more work. Same way accessing variables on stack is cheap than accessing data on heap.
- main purpose of ownership is to manage heap data and access.

Rules
1. Each value has an owner.
2. There can be only one owner at a time.
3. When owner goes out of scope, the value is dropped, unless it is moved to another variable.

Scenarios: Values can be moved and ownership can change in below scenarios
1. Variable assignment - if var is on heap or scope changes
2. Funcs - if var is on heap or scope changes
3. Funcs with return types - if var is on heap or scope changes

## rust borrowing
- borrow to use a ref.
- by default borrow is read only.
- can borrow immutable ref multiple times.
- can also borrow mutable ref, but cant use these refs simultaneously to prevent data race.
- can not have both mut and immut ref on same var within same scope.