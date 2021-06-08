use phf::phf_map;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Mode(&'static str);
impl Mode {
    pub fn share(&self) -> f64 {
        *Modes::SHARES.get(self.0).unwrap()
    }
}

pub struct Modes;
impl Modes {
    const SHARES: phf::Map<&'static str, f64> = phf_map! {
        "Feet" => 0.298f64,
        "Bike" => 0.110f64,
        "Pt" => 0.130f64,
        "CarDriver" => 0.336f64,
        "CarPassenger" => 0.126f64,
    };
}
impl IntoIterator for Modes {
    type Item = Mode;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Self::SHARES.keys().map(|key| Mode(key)).collect::<Vec<Self::Item>>().into_iter()
    }
}
