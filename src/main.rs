mod cmd;
mod basic;
mod guess;
mod setup;
mod ownership;
mod borrowing;
mod slices;
mod structure;
mod enumeration;
mod optional;
mod coll;

fn main() {
    setup::logger();
    coll::run();
    //optional::run();
    // enumeration::run();
    // structure::area_rectangle();
    // structure::basic();
    // slices::array();
    // slices::run();
    // borrowing::mutable_ref();
    // borrowing::combine_mut_unmut_ref();
    // borrowing::run();
    // ownership::vars();
    // ownership::func_with_return();
    // ownership::func_without_return();
    // basic::invalid_array_access();
    // basic::run_conditionals();
    // basic::run_functions();
    // basic::run_variables();
    // guess::run();
    // cmd::run();
}
