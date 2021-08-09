use std::any::Any;

pub struct Config {
    session:String,
    filename:String,
    test_case:Vec<dyn Any>,

}