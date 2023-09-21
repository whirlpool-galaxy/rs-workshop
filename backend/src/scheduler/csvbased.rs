use std::collections::VecDeque;
use std::collections::HashMap;

pub struct ParticipantQueue<P: super::Participant> {
    content : VecDeque<P>,

}

impl<P: super::Participant> super::ParticipantQueue<P> for ParticipantQueue<P> {
    fn dequeue(&mut self) -> P {
        self.content.pop_front().expect("Wrong type in ParticipantQueue")
    }

    fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}
impl <P: super::Participant> ParticipantQueue<P>{
    fn new()->Self{
        Self{content : VecDeque::new()}

    }
    fn enqueue(&mut self, input : P){
        self.content.push_back(input);
    }
}

pub struct Participant {
    priority_list : HashMap<super::WorkshopID, super::Priority>,
    scheduled_for : HashMap<super::TimeslotID, super::WorkshopID>,
}

impl super::Participant for Participant {
    fn get_priority_for(&self, _workshop_id: super::WorkshopID) -> super::Priority {
        *self.priority_list.get(&_workshop_id).expect("No Priority for Workshop ID found")
    }

    fn schedule(&mut self, _schedule:  &Vec<(super::TimeslotID, super::WorkshopID)>) {
        self.scheduled_for.insert(_schedule[0].0, _schedule[0].1);
    }
}

impl Participant{
    fn new()->Self{
        Self{priority_list:HashMap::new(), scheduled_for:HashMap::new()}
    }
    fn add_to_priority_list(&mut self, to_add : &Vec<(super::WorkshopID,super::Priority)>){
        self.priority_list.insert(to_add[0].0, to_add[0].1);
    }
    
}

pub struct Rooms {}

impl super::Rooms for Rooms {
    fn schedule_participant_to(
        &mut self,
        _schedule: &Vec<(super::TimeslotID, super::WorkshopID)>,
    ) -> bool {
        todo!()
    }

    fn get_available_wt(&self) -> Vec<(super::TimeslotID, Vec<super::WorkshopID>)> {
        todo!()
    }
}

#[cfg(test)]
mod tests{
    use crate::scheduler::ParticipantQueue;

    use super::*;

    #[test]
    fn testPartQueue(){
        assert!(1==2);
    }
}