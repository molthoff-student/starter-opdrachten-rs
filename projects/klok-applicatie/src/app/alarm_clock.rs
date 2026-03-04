use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use bitfield::bitfield;
use chrono::{DateTime, Local, NaiveTime, TimeZone};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

const ALARM_CLOCK_FILE: &str = "alarm_clock.json";

#[derive(Serialize, Deserialize)]
pub struct AppAlarmClock {
    pub list: Vec<AlarmClock>,
    pub selected: Option<usize>,
}

impl Default for AppAlarmClock {
    fn default() -> Self {
        Self {
            list: Vec::new(),
            selected: None,
        }
    }
}

impl AppAlarmClock {
    pub fn path() -> Result<PathBuf, Box<dyn Error>> {
        let mut path = env::current_exe()?;
        path.pop();
        path.push(ALARM_CLOCK_FILE);

        if let Some(path_str) = path.to_str() {
            println!("Settings file resides at:\n{}", path_str);
        } else {
            println!("Couldn't create path buffer.");
        }

        Ok(path)
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        let path = Self::path()?;
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader)?;
        println!("Loaded settings file.");
        Ok(data)
    }

    pub fn init() -> AppAlarmClock {
        AppAlarmClock::load().unwrap_or_else(|_| {
            let data = AppAlarmClock::default();
            println!("No settings file found. Creating and saving default...");
            data.save().expect("Couldn't initialize alarm clocks.");
            data
        })
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let path = Self::path()?;
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, self)?;
        println!("Saved alarms file.");
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct AlarmClock {
    pub name: String,
    pub time: DateTime<Local>,
    pub flags: AlarmFlags,
}

impl AlarmClock {
    pub fn new(name: String) -> Self {
        let now = Local::now();
        let alarm_time =
            NaiveTime::from_hms_opt(7, 30, 0).expect("Couldn't create NaiveTime at 7:30:00.");

        let today = now.date_naive();

        let mut date_time = Local
            .from_local_datetime(&today.and_time(alarm_time))
            .unwrap();

        if date_time <= now {
            date_time = date_time + chrono::Duration::days(1);
        }

        Self {
            name: name,
            time: date_time,
            flags: AlarmFlags(u64::MAX),
        }
    }
}

bitfield!(
    pub struct AlarmFlags(u64);
    impl Debug;

    pub enabled,    set_senabled:   0;
    pub repeats,    set_repeated:   1;

    pub monday,     set_monday:     2;
    pub tuesday,    set_tuesday:    3;
    pub wednesday,  set_wednesday:  4;
    pub thursday,   set_thursday:   5;
    pub friday,     set_friday:     6;
    pub saturday,   set_saturday:   7;
    pub sunday,     set_sunday:     8;

);

impl Serialize for AlarmFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

impl<'de> Deserialize<'de> for AlarmFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bits = u64::deserialize(deserializer)?;
        Ok(AlarmFlags(bits))
    }
}
