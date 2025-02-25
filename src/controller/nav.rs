pub enum Content {
    Daylist,
    EditTodo,
    SearchResults,
}

pub enum Widget {
    Calendar,
    EditTodo,
    Main,
    Search,
    Upcoming,
    Projects,
}

impl Widget {
    pub fn up(&self) -> Widget {
        match &self {
            Widget::Calendar => Widget::Upcoming,
            Widget::Main => Widget::Search,
            Widget::Search => Widget::Search, // do nothing
            Widget::Upcoming => Widget::Upcoming, // do nothing
            Widget::Projects => Widget::Projects, // do nothing
            _ => Widget::Main,
        }
    }
    pub fn down(&self) -> Widget {
        match &self {
            Widget::Calendar => Widget::Calendar, // do nothing
            Widget::Main => Widget::Main, // do nothing
            Widget::Search => Widget::Main,
            Widget::Upcoming => Widget::Calendar,
            Widget::Projects => Widget::Projects, // do nothing
            _ => Widget::Main,
        }
    }
    pub fn left(&self) -> Widget {
        match &self {
            Widget::Calendar => Widget::Main,
            Widget::Main => Widget::Projects,
            Widget::Search => Widget::Search, // do nothing
            Widget::Upcoming => Widget::Main,
            Widget::Projects => Widget::Projects, // do nothing
            _ => Widget::Main,
        }
    }
    pub fn right(&self) -> Widget {
        match &self {
            Widget::Calendar => Widget::Calendar, // do nothing
            Widget::Main => Widget::Calendar,
            Widget::Search => Widget::Upcoming,
            Widget::Upcoming => Widget::Upcoming, // do nothing
            Widget::Projects => Widget::Main,
            _ => Widget::Main,
        }
    }
}
