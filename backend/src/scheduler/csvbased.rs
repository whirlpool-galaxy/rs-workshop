use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;

use super::TimeslotID;
use super::WorkshopID;

pub struct ParticipantQueue<P: super::Participant> {
    content: VecDeque<P>,
}

impl<P: super::Participant> super::ParticipantQueue<P> for ParticipantQueue<P> {
    fn dequeue(&mut self) -> P {
        self.content
            .pop_front()
            .expect("Wrong type in ParticipantQueue")
    }

    fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}
impl<P: super::Participant> ParticipantQueue<P> {
    pub fn new() -> Self {
        Self {
            content: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, input: P) {
        self.content.push_back(input);
    }
}

pub struct Participant {
    priority_list: HashMap<super::WorkshopID, super::Priority>,
    scheduled_for: HashMap<super::TimeslotID, super::WorkshopID>,
}

impl super::Participant for Participant {
    fn get_priority_for(&self, _workshop_id: super::WorkshopID) -> super::Priority {
        *self
            .priority_list
            .get(&_workshop_id)
            .expect("No Priority for Workshop ID found")
    }

    fn schedule(&mut self, _schedule: &Vec<(super::TimeslotID, super::WorkshopID)>) {
        self.scheduled_for.insert(_schedule[0].0, _schedule[0].1);
    }
}

impl Participant {
    pub fn new() -> Self {
        Self {
            priority_list: HashMap::new(),
            scheduled_for: HashMap::new(),
        }
    }

    pub fn add_to_priority_list(&mut self, to_add: &Vec<(super::WorkshopID, super::Priority)>) {
        self.priority_list.insert(to_add[0].0, to_add[0].1);
    }
}

pub struct Rooms {
    available_workshops: Vec<(super::TimeslotID, Vec<super::WorkshopID>)>,
    // angenommen, die räume sind zu allen timeslots verfügbar! && raumliste ist sortiert nach raumgröße (aufsteigend) !!
    roomlist_with_capacity: Vec<(super::RoomID, i32)>,

    // these three hashmaps form together a matrix that associates a Room together with a timeslot to the according workshop (which is only unique with its timestamp)
    room_and_timeslot_to_workshop:
        HashMap<(super::TimeslotID, super::RoomID), (super::TimeslotID, super::WorkshopID)>, // i also map to timeslot and workshop id because of convenience reasons
    room_and_workshop_to_timeslot: HashMap<(super::RoomID, super::WorkshopID), super::TimeslotID>,
    timeslot_and_workshop_to_room: HashMap<(super::TimeslotID, super::WorkshopID), super::RoomID>,

    // how many people are already visiting the workshop
    workshop_occupancy: HashMap<(super::TimeslotID, super::WorkshopID), i32>,
}
impl Rooms {
    pub fn new() -> Self {
        Self {
            available_workshops: Vec::new(),
            roomlist_with_capacity: Vec::new(),
            room_and_timeslot_to_workshop: HashMap::new(),
            room_and_workshop_to_timeslot: HashMap::new(),
            timeslot_and_workshop_to_room: HashMap::new(),
            workshop_occupancy: HashMap::new(),
        }
    }

    pub fn add_available_workshop(
        &mut self,
        timeslot: super::TimeslotID,
        workshop_id: super::WorkshopID,
    ) {
        for avail_workshops in &mut self.available_workshops {
            if avail_workshops.0 == timeslot {
                avail_workshops.1.push(workshop_id);
                return;
            }
        }
        let newvec = vec![workshop_id];
        self.available_workshops.push((timeslot, newvec));
    }

    // tries to swap a with b, if occupancy of a is bigger than occupancy of b AND both a and b fit into the other room, returns true if swapped
    // assumes that b is in a bigger room than a, and timeslot of a and b are the same
    fn try_swap_workshops(
        &mut self,
        workshop_a: (super::TimeslotID, super::WorkshopID),
        workshop_b: (super::TimeslotID, super::WorkshopID),
    ) -> bool {
        let room_a = *self
            .timeslot_and_workshop_to_room
            .get(&workshop_a)
            .expect("roomid");
        let room_b = *self
            .timeslot_and_workshop_to_room
            .get(&workshop_b)
            .expect("roomid");
        let capacity_a = self.roomlist_with_capacity[self
            .roomlist_with_capacity
            .iter()
            .position(|&x| x.0 == room_a)
            .unwrap()]
        .1;
        let capacity_b = self.roomlist_with_capacity[self
            .roomlist_with_capacity
            .iter()
            .position(|&x| x.0 == room_b)
            .unwrap()]
        .1;
        let occupancy_a = *self.workshop_occupancy.get(&workshop_a).expect("occupancy");
        let occupancy_b = *self.workshop_occupancy.get(&workshop_b).expect("occupancy");

        if occupancy_a > occupancy_b {
            // swap is recommended
            if occupancy_a <= capacity_b && occupancy_b <= capacity_a {
                // able to swap
                self.room_and_timeslot_to_workshop
                    .insert((room_a, workshop_b.0), workshop_b);
                self.room_and_timeslot_to_workshop
                    .insert((room_b, workshop_a.0), workshop_a);
                self.room_and_workshop_to_timeslot
                    .insert((room_a, workshop_b.1), workshop_b.0);
                self.room_and_workshop_to_timeslot
                    .insert((room_b, workshop_a.1), workshop_a.0);
                self.timeslot_and_workshop_to_room
                    .insert(workshop_b, room_a);
                self.timeslot_and_workshop_to_room
                    .insert(workshop_a, room_b);
                return true;
            }
        }
        return false;
    }

    fn give_workshop_a_bigger_room(
        &mut self,
        workshop_and_timeslot: (super::TimeslotID, super::WorkshopID),
    ) -> bool {
        let room_id: super::RoomID = *self
            .timeslot_and_workshop_to_room
            .get(&workshop_and_timeslot)
            .expect("no valid roomid");
        let index = self
            .roomlist_with_capacity
            .iter()
            .position(|&x| x.0 == room_id)
            .unwrap();
        if index + 1 >= self.roomlist_with_capacity.len() {
            // no bigger room found
            return false;
        }
        // this loop invokes reallocate
        loop {
            let in_use = self.room_and_timeslot_to_workshop.get(&(
                self.roomlist_with_capacity
                    .get(index + 1)
                    .expect("index in roomlist not found")
                    .0,
                workshop_and_timeslot.1,
            ));
            if in_use.is_none() {
                // room is not in use
                todo!(); // assign me!!
                return true;
            }
            let next_workshop = *in_use.unwrap();
            let next_workshop_occupancy = *self.workshop_occupancy.get(&next_workshop).unwrap();
            todo!(); // full bubble sort of all bigger rooms and their workshops needed !!
            if next_workshop_occupancy <= self.roomlist_with_capacity[index].1 {
                todo!(); // swap is possible
                         // change the workshop rooms
                return true;
            }
        }
    }
}

impl super::Rooms for Rooms {
    // gets a schedule of one participant, assigns his workshops, and if the workshops get to full --> reallocate the workshops room, if no room can be found --> return false
    fn schedule_participant_to(
        &mut self,
        _schedule: &Vec<(super::TimeslotID, super::WorkshopID)>,
    ) -> bool {
        return false;
    }

    // returns for every time slot all still available workshop (no schedule gone wrong for the workshop)
    fn get_available_wt(&self) -> Vec<(super::TimeslotID, Vec<super::WorkshopID>)> {
        return self.available_workshops.clone();
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::scheduler::csvbased::Rooms;
    use crate::scheduler::ParticipantQueue;
    use crate::scheduler::Rooms as RoomTrait;

    use super::*;

    #[test]
    fn test_part_queue() {
        assert!(1 == 2);
    }

    #[test]
    fn test_room_available_workshops() {
        let mut room = Rooms::new();
        assert!(room.get_available_wt().is_empty());
        room.add_available_workshop(1, 69);
        assert!(!room.get_available_wt().is_empty());
        room.add_available_workshop(1, 420);
        assert_eq!(room.get_available_wt(), vec![(1, vec![69, 420])]);
    }
}
