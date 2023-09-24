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

/// selection must be ordered so that a index of selection maps to the timeslot of the workshop
fn optimise_rec(
    workshops: &Vec<(TimeslotID, Vec<WorkshopID>)>,
    participant: &dyn Participant,
    selection: &mut Vec<WorkshopID>,
    max_score: &mut (Priority, Vec<(TimeslotID, WorkshopID)>),
) {
    if selection.len() == workshops.len() {
        // recursion base case

        let mut score = 0; // priority score `selection` case.
        let mut r_vec = vec![]; // schedule vector

        // iterate over selected workshops, calculate score and create time table
        for (idx, w) in selection.into_iter().enumerate() {
            score += participant.get_priority_for(*w);

            r_vec.push((
                workshops
                    .get(idx)
                    .expect("Internal Error at optimise_rec")
                    .0, // get TimeslotID for workshop
                *w,
            ));
        }

        if score > max_score.0 {
            // update score
            *max_score = (score, r_vec);
        }
    } else {
        // recursion step

        // get level of recursive decent
        let s = selection.len();

        // get available workshops
        let workshop_pt = &workshops.get(s).expect("Internal Error at optimise_rec").1;

        for w in workshop_pt {
            if !selection.contains(w) {
                // check if workshop already in selection

                // add workshop to selection
                selection.push(*w);
                // recursive decent
                optimise_rec(workshops, participant, selection, max_score);
                // remove workshop from select, if score was optimal the selection is stored in max_score and can be destroyed hier.
                selection.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{optimise_rec, Priority, WorkshopID};

    struct Participant {
        hash_map: HashMap<WorkshopID, Priority>,
    }

    impl Participant {
        pub fn new(hash_map: HashMap<WorkshopID, Priority>) -> Self {
            Self { hash_map }
        }
    }

    impl super::Participant for Participant {
        fn get_priority_for(&self, workshop_id: WorkshopID) -> Priority {
            *self.hash_map.get(&workshop_id).unwrap()
        }

        fn schedule(&mut self, _schedule: &Vec<(super::TimeslotID, WorkshopID)>) {
            unimplemented!()
        }
    }

    #[test]
    fn optimise_rec_test() {
        let workshops = vec![(1, vec![1, 2, 3]), (2, vec![3, 5, 6])];
        let mut hm = HashMap::new();
        hm.insert(1, 5);
        hm.insert(2, 4);
        hm.insert(3, 3);
        hm.insert(5, 2);
        hm.insert(6, 1);
        let participant = Participant::new(hm);

        let mut selection = vec![];
        let mut max_score = (0, vec![]);

        optimise_rec(&workshops, &participant, &mut selection, &mut max_score);

        dbg!(&max_score);
    }
}
