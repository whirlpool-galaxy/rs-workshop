# API DOC

## Participant

`/participant`

### Sign In

`POST /signin` Sign in participants. No authorisation required. Cookie with authorisation information returned on success.
```json
{
    "username": "<USERNAME>",
    "initial": "<INITIAL ACCESS CODE>",
}
```
If sign in fails: `401 Unauthorized`.

### Workshops


`GET /workshops/list` Get all workshops with names and description.
```json
{
    "workshops":
    [
        {
            "id": 0123,
            "name": "<NAME>",
            "description": "<DESCRIPTION>",
        },
    ],
}
```
If workshop selection closed: `403 Forbidden`.

`GET /workshops/selection` Get selected workshops for current participant.
```json
{
    "workshops": [
        {
            "id": 0123,
            "name": "<NAME>",
            "description": "<DESCRIPTION>",
            "timeslot": "<START TIME>",
            "room": "<ROOM>"
        }
    ],
}
```
If workshop selection results not published yet: `403 Forbidden`.

`POST /workshop/priority` Submit current participants priority list to the server. Requires a unique priority for every workshop.
```json
{
    "selection": [
        {
            "id": 0123,
            "priority": 0123,
        },
    ],
}
```
If priorities are committed, fails with: `403 Forbidden`.

`POST /workshop/priority/commit` Commits the priorities the current participants has chosen.

## Admin

`/admin`

`POST /signin` Sign in admins. No authorisation required. Cookie with authorisation information returned on success.
```json
{
    "username": "<USERNAME>",
    "passwd": "<PASSWORD>",
}
``` 
If sign in fails: `401 Unauthorized`.


`POST /selection/open` Participants can now sign in and select workshops.

`POST /selection/close` All participants are unable to change their selection.

`POST /selection/publish` Participants can now see the result of their selection.

`POST /accesscodes/generate/{number of access codes}` Generate initial access codes and return them.

`GET /accesscodes/batch/{batch number}` Returns all access codes of generation batch `batch number`.`

`GET /accesscodes/all` Return all access codes.

`GET /accesscode/{username}` Returns the access code of username.

`GET /schedule/rooms`
```json
{
    "schedule": [
        {
            "timeslot": "<START TIME>",
            "workshop": "<WORKSHOP NAME>",
            "room": "<ROOM>",
            "capacity": "<CAPACITY>",
            "utilisation": "<EXPECTED NUMBER OF PARTICIPANTS>",
            "participants": [
                "<PARTICIPANT USERNAME>",
                "..."
            ],
        },
    ],
}
```
