use std::collections::VecDeque;
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

pub struct Participant {}

impl super::Participant for Participant {
    fn get_priority_for(&self, _workshop_id: super::WorkshopID) -> super::Priority {
        todo!()
    }

    fn schedule(&mut self, _schedule: &Vec<(super::TimeslotID, super::WorkshopID)>) {
        todo!()
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