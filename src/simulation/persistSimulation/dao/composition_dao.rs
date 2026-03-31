use crate::models::CompositionMatch;

pub trait CompositionDao {
    fn find_by_match_and_club(
        &self,
        match_id: i32,
        club_id: i32,
    ) -> Result<Option<CompositionMatch>, String>;
}