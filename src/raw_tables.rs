use std::time::Instant;

use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Participant {
    pub participant_id: i32,
    pub initial_access_code: Option<String>, // 10
    pub username: Option<String>,            // 64
}

#[derive(Debug, FromRow)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub display_name: Option<String>,
    pub salt: Option<String>, // 32
    pub hash: Option<String>, // 64
    pub admin: bool,
}

#[derive(Debug, FromRow)]
pub struct RoomRequirement {
    pub requirement_id: i32,
    pub name: String, // 32
    pub description: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct Workshop {
    pub workshop_id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct WorkshopRequires {
    pub workshop_id: i32,
    pub requirement_id: i32,
}

#[derive(Debug, FromRow)]
pub struct Room {
    pub room_id: i32,
    pub name: String,
    pub location: String,
}

#[derive(Debug, FromRow)]
pub struct RoomHas {
    pub room_id: i32,
    pub requirement_id: i32,
}

#[derive(Debug, FromRow)]
pub struct TimeSlot {
    pub timeslot_id: i32,
    pub begin: Instant,
    pub end: Instant,
}

#[derive(Debug, FromRow)]
pub struct WorkshopSchedule {
    pub workshop_id: i32,
    pub timeslot_id: i32,
}

#[derive(Debug, FromRow)]
pub struct RoomAvailable {
    pub room_id: i32,
    pub timeslot_id: i32,
}

#[derive(Debug, FromRow)]
pub struct WorkshopResponsible {
    pub user_id: i32,
    pub workshop_id: i32,
}

#[derive(Debug, FromRow)]
pub struct WorkshopRoomSchedule {
    pub schedule_id: i32,
    pub workshop_id: i32,
    pub room_id: i32,
    pub timeslot_id: i32,
}

#[derive(Debug, FromRow)]
pub struct ParticipantSchedule {
    pub participant_id: i32,
    pub schedule_id: i32,
}

#[derive(Debug, FromRow)]
pub struct PrioritySelection {
    pub participant_id: i32,
    pub workshop_id: i32,
    pub priority: i32,
}
