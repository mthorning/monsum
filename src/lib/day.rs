pub enum Period {
    Morning,
    Afternoon,
    Evening,
}

pub struct Habit {
    pub value: String,
    pub completed: bool,
    pub period: Period,
}

pub struct Day {
    pub date: String,
    pub events: Vec<String>,
    pub habits: Vec<Habit>,
    pub notes: Vec<String>,
    pub gratitudes: Vec<String>,
}

