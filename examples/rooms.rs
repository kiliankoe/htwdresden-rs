extern crate htwdresden;

use htwdresden::{Week, Weekday, Room, Building};

fn main() {
    let rooms = Room::get_free(Week::Even, Weekday::Monday, "9:30", "10:30", Building::Z);
    println!("{:?}", rooms);
}
