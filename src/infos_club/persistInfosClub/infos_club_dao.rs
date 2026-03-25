use crate::models::InfosClub;
use rusqlite::Result;

pub trait InfosClubDAO {
    fn get_infos_by_club(&self, id: i32) -> Result<InfosClub>;
}
 