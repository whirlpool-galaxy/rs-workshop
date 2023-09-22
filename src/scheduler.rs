pub mod csvbased;
pub mod dbbased;

pub type ParticipantID = i32;
pub type WorkshopID = i32;
pub type TimeslotID = i32;
pub type RoomID = i32;
pub type Priority = i32;

pub trait ParticipantQueue<P: Participant> {
    fn dequeue(&mut self) -> P;
    fn is_empty(&self) -> bool;
}

pub trait Participant {
    fn get_priority_for(&self, workshop_id: WorkshopID) -> Priority;
    fn schedule(&mut self, schedule: &Vec<(TimeslotID, WorkshopID)>);
}

pub trait Rooms {
    fn schedule_participant_to(&mut self, schedule: &Vec<(TimeslotID, WorkshopID)>) -> bool;
    fn get_available_wt(&self) -> Vec<(TimeslotID, Vec<WorkshopID>)>;
}

pub fn schedule(participants: &mut dyn ParticipantQueue<impl Participant>, rooms: &mut dyn Rooms) {
    let mut avail = rooms.get_available_wt();
    while !participants.is_empty() {
        let mut p = participants.dequeue();
        loop {
            let schedule = optimise(&avail);
            if rooms.schedule_participant_to(&schedule) {
                p.schedule(&schedule);
                break;
            } else {
                avail = rooms.get_available_wt();
            }
        }
    }
}

pub fn optimise(_workshops: &Vec<(TimeslotID, Vec<WorkshopID>)>) -> Vec<(TimeslotID, WorkshopID)> {
    vec![(0, 0)]
}
