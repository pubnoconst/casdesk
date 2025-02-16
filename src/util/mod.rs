use std::rc::Rc;

pub mod renderer;

trait ShortString {
    fn cut_30(&self) -> String;
}

impl ShortString for &str {
    fn cut_30(&self) -> String {
        self.chars().take(30).collect()
    }
}

trait LongString {
    fn cut_80(&self) -> String;
}

impl LongString for &str {
    fn cut_80(&self) -> String {
        self.chars().take(100).collect()
    }
}

#[derive(Debug)]
pub struct Customer {
    name: Rc<str>,
    contact: Rc<str>,
    address: Rc<str>,
    id_num: Rc<str>
}

impl Customer {
    pub fn new(name: &str, contact: &str, address: &str, id_num: &str) -> Self {
        Customer {
            name: name.cut_30().into(),
            contact: contact.cut_30().into(),
            address: address.cut_80().into(),
            id_num: id_num.cut_30().into()
        }
    }
}

#[derive(Debug)]
pub struct SellableDevice {
    name: Rc<str>,
    color: Rc<str>,
    locked: Rc<str>,
    imei: Rc<str>,
    price: Rc<str>,
}

impl SellableDevice {
    pub fn new(name: &str, color: &str, locked: &str, imei: &str, price: &str) -> Self {
        Self {
            name: name.cut_30().into(),
            color: color.cut_30().into(),
            locked: locked.cut_30().into(),
            imei: imei.cut_30().into(),
            price: price.cut_30().into(),
        }
    }
}

#[derive(Debug)]
pub struct PurchasedDevice {
    name: Rc<str>,
    color: Rc<str>,
    memory: Rc<str>,
    locked: Rc<str>,
    imei: Rc<str>,
}

impl PurchasedDevice {
    pub fn new(name: &str, color: &str, memory: &str, locked: &str, imei: &str) -> Self {
        Self {
            name: name.cut_30().into(),
            color: color.cut_30().into(),
            memory: memory.cut_30().into(),
            locked: locked.cut_30().into(),
            imei: imei.cut_30().into(),
        }
    }
}

#[derive(Debug)]
pub struct LeasedDevice {
    name: Rc<str>,
    storage: Rc<str>,
    color: Rc<str>,
    imei: Rc<str>,
    condition: Rc<str>
}

impl LeasedDevice {
    pub fn new(name: &str, storage: &str, color: &str, imei: &str, condition: &str) -> Self {
        Self {
            name: name.cut_30().into(),
            storage: storage.cut_30().into(),
            color: color.cut_30().into(),
            imei: imei.cut_30().into(),
            condition: condition.cut_30().into()
        }
    }
}


mod date {
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn get_today() -> String {
        // Get the current system time
        let now = SystemTime::now();

        // Calculate the duration since the UNIX epoch
        let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

        // Convert the duration to seconds
        let secs = duration.as_secs();

        // Calculate the number of days since the epoch, adjusting for the current time of day
        let days_since_epoch = (secs + 86400 / 2) / 86400; // Add 12 hours to round to the nearest day

        // Calculate the current year, month, and day
        let (year, month, day) = days_to_ymd(days_since_epoch);

        // Format the date as DD/MM/YYYY
        format!("{:02}/{:02}/{:04}", day, month, year)
    }

    fn days_to_ymd(mut days_since_epoch: u64) -> (u64, u64, u64) {
        let mut year = 1970;
        let mut month = 1;

        // Iterate through years
        loop {
            let is_leap_year = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
            let days_in_year = if is_leap_year { 366 } else { 365 };

            if days_since_epoch >= days_in_year {
                days_since_epoch -= days_in_year;
                year += 1;
            } else {
                break;
            }
        }

        // Iterate through months
        let is_leap_year = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let days_in_month = [
            31,
            if is_leap_year { 29 } else { 28 },
            31,
            30,
            31,
            30,
            31,
            31,
            30,
            31,
            30,
            31,
        ];

        for &days in &days_in_month {
            if days_since_epoch >= days {
                days_since_epoch -= days;
                month += 1;
            } else {
                break;
            }
        }

        // Calculate the day
        let day = days_since_epoch + 1;

        (year, month, day)
    }
}
