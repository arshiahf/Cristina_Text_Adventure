extern crate user_input;
extern crate text_adventure;

use user_input::requests::{request_input_string};
//use text_adventure::commands::{interaction,perception};
use text_adventure::environment::{create_rooms};

fn main()
{
    create_rooms();
}
