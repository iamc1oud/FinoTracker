

enum WeekDays {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY
}

pub struct FinanceSettings {
    week_day: WeekDays    
}

impl  FinanceSettings {
    pub fn new() -> Self{
        return FinanceSettings{
            week_day: WeekDays::MONDAY
        }
    }
}