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
            let schedule = optimise(&avail, &p);
            if rooms.schedule_participant_to(&schedule) {
                p.schedule(&schedule);
                break;
            } else {
                avail = rooms.get_available_wt();
            }
        }
    }
}

pub fn optimise(
    workshops: &Vec<(TimeslotID, Vec<WorkshopID>)>,
    participant: &dyn Participant,
) -> Vec<(TimeslotID, WorkshopID)> {
    let mut selection = vec![];
    let mut max_score = (0, vec![]);

    optimise_rec(workshops, participant, &mut selection, &mut max_score);

    assert_ne!(max_score.0, 0, "Internal Error at optimise");

    max_score.1
}

fn optimise_rec(
    workshops: &Vec<(TimeslotID, Vec<WorkshopID>)>,
    participant: &dyn Participant,
    selection: &mut Vec<WorkshopID>,
    max_score: &mut (Priority, Vec<(TimeslotID, WorkshopID)>),
) {
    if selection.len() == workshops.len() {
        let mut score = 0;

        let mut r_vec = vec![];

        for (idx, w) in selection.into_iter().enumerate() {
            score += participant.get_priority_for(*w);

            r_vec.push((
                workshops
                    .get(idx)
                    .expect("Internal Error at optimise_rec")
                    .0,
                *w,
            ));
        }

        if score > max_score.0 {
            max_score.0 = score;
            max_score.1 = r_vec;
        }
    } else {
        let s = selection.len() - 1;

        let workshop_pt = match workshops.get(s) {
            Some(w) => &w.1,
            None => panic!("Internal Error at optimise_rec"),
        };

        for w in workshop_pt {
            if !selection.contains(w) {
                selection.push(*w);
                optimise_rec(workshops, participant, selection, max_score);
                selection.pop();
            }
        }
    }
}
