mod git;
mod shadow;
mod channel;

fn main() {
    println!("{:?}",shadow::SystemEnv::new());

    //     use std::env;
    // for (key, value) in env::vars() {
    //     println!("{}: {}", key, value);
    // }
    //     println!("{:?}", env::var("RUSTC_WRAPPER"));
}