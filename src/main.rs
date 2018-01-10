extern crate user_input;
extern crate text_adventure;

//use user_input::requests::{request_input_string, request_input_to_vec};
//use text_adventure::commands::{interaction,perception};
use text_adventure::environment::{create_rooms};

fn main()
{
    /*
    let mut vec_in:Vec<String> = Vec::new();
    request_input_to_vec("Write a series of strings, separated by white space: ", &mut vec_in);
    let mut str_in:String = String::new();
    request_input_string("Write a string: ", &mut str_in);

    for item in vec_in
    {
        println!("{}", item);
    }
    println!("{}", str_in);
    */
    let root_dir:String = String::from("rooms\\");
    create_rooms(root_dir);

}
