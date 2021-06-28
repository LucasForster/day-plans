use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Purpose {
    Home,
    Leisure,
    Work,
    School,
    Service,
    Shopping,
}
impl FromStr for Purpose {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Arbeit" => Ok(Self::Work),
            "Dienstleistung" => Ok(Self::Service),
            "Einkaufen" => Ok(Self::Shopping),
            "Freizeit" => Ok(Self::Leisure),
            "Grundschule" => Ok(Self::School),
            "Hörsaal" => Ok(Self::School),
            "HörsaalHin" => Ok(Self::School),
            "Hörsaalplatz" => Ok(Self::School),
            "HörsaalRück" => Ok(Self::School),
            "Service" => Ok(Self::Service),
            "Stud.Ziele" => Ok(Self::School),
            "weiterf.Schule" => Ok(Self::School),
            "Wohnen" => Ok(Self::Home),
            unknown => Err(format!("Unknown purpose string \"{}\"!", unknown)),
        }
    }
}
impl Purpose {
    pub fn duration(&self) -> Duration {
        macro_rules! hours {
            ($h:expr) => {
                minutes!($h * 60)
            };
        }
        macro_rules! minutes {
            ($m:expr) => {
                Duration::from_secs($m * 60)
            };
        }
        match self {
            Self::Home => hours!(8),
            Self::Work => hours!(8),
            Self::School => hours!(7),
            Self::Leisure => hours!(2),
            Self::Shopping => hours!(1),
            Self::Service => minutes!(30),
        }
    }
}
