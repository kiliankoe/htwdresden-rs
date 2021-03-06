use Error;
use Week;
use Weekday;
use Building;

use reqwest;

#[derive(Debug)]
pub struct Room {
    pub name: String,
}

// The API spits out a list of strings on this request, so we can't deserialize
// the room values directly, but have to convert afterwards.
impl From<String> for Room {
    fn from(string: String) -> Self {
        Room { name: string }
    }
}

const BASE_URL: &'static str = "https://www2.htw-dresden.de/~app/API/GetFreeRooms.php";

impl Room {
    /// Returns a list of free `Rooms`s for a list of given parameters.
    ///
    /// # Arguments
    ///
    /// * `week` - Even or odd week
    /// * `Weekday` - A specified weekday
    /// * `start_time` - Beginning of the search timeframe
    /// * `end_time` - Ending of the search timeframe
    /// * `building` - Which building to search in
    ///
    /// # Example
    ///
    /// ```
    /// use htwdresden::{Week, Weekday, Room, Building};
    ///
    /// let rooms = Room::get_free(Week::Even, Weekday::Monday, "9:30", "10:30", Building::Z);
    /// ```
    pub fn get_free(week: Week,
                    day: Weekday,
                    start_time: &str,
                    end_time: &str,
                    building: Building)
                    -> Result<Vec<Room>, Error> {
        let url = format!("{}?week={}&day={}&startTime={}&endTime={}&building={:?}",
                          BASE_URL,
                          week as u8,
                          day as u8,
                          start_time,
                          end_time,
                          building);
        let rooms = reqwest::get(&url)
            ?
            .json()
            .map(|response: Vec<String>| response)?
            .iter()
            .map(|room| room.to_string().into())
            .collect();

        Ok(rooms)
    }
}
