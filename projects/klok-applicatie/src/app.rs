pub mod alarm_clock;
pub use alarm_clock::*;
pub mod analog_clock;
pub use analog_clock::*;
pub mod settings;
pub use settings::*;

pub struct AppData {
    pub alarm_clock: AppAlarmClock,
    pub settings: AppSettings,
}

impl AppData {
    pub fn init() -> Self {
        Self {
            alarm_clock: AppAlarmClock::init(),
            settings: AppSettings::init(),
        }
    }
}
