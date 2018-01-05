extern crate user_input;
//extern crate text_adventure;

use user_input::requests::{request_input};
//use text_adventure::commands::{interaction,perception};
//use text_adventure::environment;

fn main()
{
    print!("Write something: ");
    request_input();
}
