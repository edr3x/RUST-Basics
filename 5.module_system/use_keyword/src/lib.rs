mod front_of_house; //* same name as filename */
//? not ideal approach */
pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}

//? using use keyword

// use crate::front_of_house::hosting; //* absolute path */
use front_of_house::hosting; //* Relative path */
pub fn eat_at_restaurant1() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
