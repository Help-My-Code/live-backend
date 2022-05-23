struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Room {
    name: String,
    users: Vec<User>,
}

struct Message {
    sender: User,
    room: Room,
    content: String,
}

struct UpdateCode {
    content: String,
    user: User,
    room: Room,
}

struct Session {
    user: User,
    active: bool,
    room: Room,
    name: String,
}