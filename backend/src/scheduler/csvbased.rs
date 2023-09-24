use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

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
    room_of_workshop_at_timeslot: HashMap<(super::TimeslotID, super::WorkshopID), super::RoomID>,
    room_occupancy_at_timeslot: HashMap<(super::RoomID, super::TimeslotID), i32>,
    available_workshops: Vec<(super::TimeslotID, Vec<super::WorkshopID>)>,
    // angenommen, die räume sind zu allen timeslots verfügbar!
    roomlist_with_capacity: HashMap<super::RoomID, i32>,
}
impl Rooms {
    pub fn new() -> Self {
        Self {
            room_of_workshop_at_timeslot: HashMap::new(),
            room_occupancy_at_timeslot: HashMap::new(),
            available_workshops: Vec::new(),
            roomlist_with_capacity: HashMap::new(),
        }
    }

    fn insert_tuple_roomlist(
        &mut self,
        room_id: super::RoomID,
        workshop_timeslot_tuple: (super::WorkshopID, super::TimeslotID),
    ) {
        self.room_of_workshop_at_timeslot
            .insert(workshop_timeslot_tuple, room_id);
    }

    fn remove_tuple_roomlist(
        &mut self,
        workshop_timeslot_tuple: (super::WorkshopID, super::TimeslotID),
    ) {
        self.room_of_workshop_at_timeslot
            .remove(&workshop_timeslot_tuple);
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
}

impl super::Rooms for Rooms {
    // gets a schedule of one participant, assigns his workshops, and if the workshops get to full --> reallocate the workshops room, if no room can be found --> return false
    fn schedule_participant_to(
        &mut self,
        _schedule: &Vec<(super::TimeslotID, super::WorkshopID)>,
    ) -> bool {
        // create copy to work with, so in case of failure changes will be discarded
        let copy_of_room_of_workshop_and_timeslot = self.room_of_workshop_at_timeslot.clone();

        for selection in _schedule {
            todo!(); // Problem is: if a workshop has no room, this would panic. I have to choose a design that either 1) initialises each workshop with a room, or 2) change code here so if a workshop has no room it gets one
            let room: super::RoomID = *self
                .room_of_workshop_at_timeslot
                .get(selection)
                .expect("Expected RoomID");
            if (*(self
                .room_occupancy_at_timeslot
                .get(&(room, selection.1))
                .expect("Room Occupancy expected"))
                + 1)
                < *self
                    .roomlist_with_capacity
                    .get(&room)
                    .expect("Expected a Capacity")
            {
                // in this case, the capacity of the room is big enough to contain an additional participant, so we just add him to the occupancy
            }
        }

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
