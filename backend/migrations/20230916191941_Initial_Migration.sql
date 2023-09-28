CREATE TABLE session (
	"key" bytea NOT NULL,
	val text NOT NULL,
	CONSTRAINT session_pk PRIMARY KEY ("key")
);

CREATE TABLE participant (
	participant_id serial NOT NULL,
	initial_access_code char(10) NULL,
	username char(64) NULL,
	batch int NOT NULL,
	"committed" bool NOT NULL DEFAULT false,
	CONSTRAINT participant_pk PRIMARY KEY (participant_id)
);

CREATE TABLE "user" (
	user_id serial NOT NULL,
	username varchar NOT NULL,
	display_name varchar NULL,
	salt char(32) NULL,
	hash char(64) NULL,
	"admin" bool NOT NULL DEFAULT false,
    CONSTRAINT user_pk PRIMARY KEY (user_id),
    CONSTRAINT user_un UNIQUE(username)
);

CREATE TABLE room_requirement (
	requirement_id serial NOT NULL,
	"name" char(32) NOT NULL,
	description varchar NULL,
	CONSTRAINT room_requirement_pk PRIMARY KEY (requirement_id),
	CONSTRAINT room_requirement_un UNIQUE ("name")
);

CREATE TABLE workshop (
	workshop_id serial NOT NULL,
	"name" varchar NOT NULL,
	description varchar NULL,
	max_capacity int NULL,
	CONSTRAINT workshop_pk PRIMARY KEY (workshop_id),
	CONSTRAINT workshop_un UNIQUE ("name")
);

CREATE TABLE workshop_requires (
	workshop_id int NOT NULL,
	requirement_id int NOT NULL,
	CONSTRAINT workshop_requires_pk PRIMARY KEY (workshop_id,requirement_id),
	CONSTRAINT workshop_requires_fk_0 FOREIGN KEY (workshop_id) REFERENCES workshop(workshop_id),
	CONSTRAINT workshop_requires_fk_1 FOREIGN KEY (requirement_id) REFERENCES room_requirement(requirement_id)
);

CREATE TABLE room (
	room_id serial NOT NULL,
	"name" varchar NOT NULL,
	"location" varchar NOT NULL,
	CONSTRAINT room_pk PRIMARY KEY (room_id)
);

CREATE TABLE room_has (
	room_id int NOT NULL,
	requirement_id int NOT NULL,
	CONSTRAINT room_has_pk PRIMARY KEY (room_id,requirement_id),
	CONSTRAINT room_has_fk_0 FOREIGN KEY (room_id) REFERENCES room(room_id),
	CONSTRAINT room_has_fk_1 FOREIGN KEY (requirement_id) REFERENCES room_requirement(requirement_id)
);

CREATE TABLE timeslot (
	timeslot_id serial NOT NULL,
	"begin" timestamp NOT NULL,
	"end" timestamp NOT NULL,
	CONSTRAINT timeslot_pk PRIMARY KEY (timeslot_id)
);

CREATE TABLE workshop_schedule (
	workshop_id int NOT NULL,
	timeslot_id int NOT NULL,
	CONSTRAINT workshop_schedule_pk PRIMARY KEY (workshop_id,timeslot_id),
	CONSTRAINT workshop_schedule_fk_0 FOREIGN KEY (workshop_id) REFERENCES workshop(workshop_id),
	CONSTRAINT workshop_schedule_fk_1 FOREIGN KEY (timeslot_id) REFERENCES timeslot(timeslot_id)
);

CREATE TABLE room_available (
	room_id int NOT NULL,
	timeslot_id int NOT NULL,
	CONSTRAINT room_available_pk PRIMARY KEY (timeslot_id,room_id),
	CONSTRAINT room_available_fk_0 FOREIGN KEY (room_id) REFERENCES room(room_id),
	CONSTRAINT room_available_fk_1 FOREIGN KEY (timeslot_id) REFERENCES timeslot(timeslot_id)
);

CREATE TABLE workshop_responsible (
	user_id int NOT NULL,
	workshop_id int NOT NULL,
	CONSTRAINT workshop_responsible_pk PRIMARY KEY (user_id,workshop_id),
	CONSTRAINT workshop_responsible_fk_0 FOREIGN KEY (user_id) REFERENCES "user"(user_id),
	CONSTRAINT workshop_responsible_fk_1 FOREIGN KEY (workshop_id) REFERENCES workshop(workshop_id)
);

CREATE TABLE workshop_room_schedule (
    schedule_id serial NOT NULL,
	workshop_id int NOT NULL,
	room_id int NOT NULL,
	timeslot_id int NOT NULL,
    CONSTRAINT workshop_room_schedule_pk PRIMARY KEY (schedule_id),
	CONSTRAINT workshop_room_schedule_un UNIQUE (workshop_id,room_id,timeslot_id),
	CONSTRAINT workshop_room_schedule_fk_0 FOREIGN KEY (workshop_id) REFERENCES workshop(workshop_id),
	CONSTRAINT workshop_room_schedule_fk_1 FOREIGN KEY (room_id) REFERENCES room(room_id),
	CONSTRAINT workshop_room_schedule_fk_2 FOREIGN KEY (timeslot_id) REFERENCES timeslot(timeslot_id)
);

CREATE TABLE participant_schedule (
    participant_id int NOT NULL,
    schedule_id int NOT NULL,
    CONSTRAINT participant_schedule_pk PRIMARY KEY (participant_id, schedule_id),
    CONSTRAINT participant_schedule_fk_0 FOREIGN KEY (participant_id) REFERENCES participant(participant_id),
    CONSTRAINT participant_schedule_fk_1 FOREIGN KEY (schedule_id) REFERENCES workshop_room_schedule(schedule_id)
);

CREATE TABLE priority_selection (
	participant_id int NOT NULL,
	workshop_id int NOT NULL,
	priority int NOT NULL,
	CONSTRAINT priority_selection_pk PRIMARY KEY (participant_id,workshop_id),
	CONSTRAINT priority_selection_fk_0 FOREIGN KEY (participant_id) REFERENCES participant(participant_id),
	CONSTRAINT priority_selection_fk_1 FOREIGN KEY (workshop_id) REFERENCES workshop(workshop_id)
);