pub enum Mode {
    Feet, Bike, Pt, CarDriver, CarPassenger
}
impl Mode {
    pub fn get_share(&self) -> f64 {
        /*
         * Source: "Statistisches Jahrbuch", Stadt Aachen, 2017, p.104
         * Copyright: Stadt Aachen FB02/200
         * License: "Nachdruck oder weitere Veröffentlichung mit Quellenangabe gestattet"
         */
        match *self {
            Mode::Feet => 0.298,
            Mode::Bike => 0.110,
            Mode::Pt => 0.130,
            Mode::CarDriver => 0.336,
            Mode::CarPassenger => 0.126,
        }
    }
}
