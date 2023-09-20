pub struct ParticipantQueue<P: super::Participant> {
    _p: P,
}

impl<P: super::Participant> super::ParticipantQueue<P> for ParticipantQueue<P> {
    fn dequeue(&mut self) -> P {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
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

    fn get_available_wt(&self) -> Vec<(super::TimeslotID, super::WorkshopID)> {
        todo!()
    }
}
