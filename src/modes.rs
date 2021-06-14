use super::trips::Transport;
use lazy_static::lazy_static;

pub struct Mode {
    pub index: usize,
    pub name: &'static str,
    pub share: f64,
    pub transport: Transport,
    _priv: (),
}

pub const COUNT: usize = 5;

lazy_static! {
    pub static ref MODES: Vec<Mode> = {
        let mut vec: Vec<Mode> = Vec::new();
        /*
        * Source: "Statistisches Jahrbuch", Stadt Aachen, 2017, p.104
        * Copyright: Stadt Aachen FB02/200
        * License: "Nachdruck oder weitere Veröffentlichung mit Quellenangabe gestattet"
        */
        vec.push(Mode { index: vec.len(), name: "Feet", share: 0.298, transport: Transport::Individual, _priv: () });
        vec.push(Mode { index: vec.len(), name: "Bike", share: 0.110, transport: Transport::Individual, _priv: () });
        vec.push(Mode { index: vec.len(), name: "Pt", share: 0.130, transport: Transport::Public, _priv: () });
        vec.push(Mode { index: vec.len(), name: "CarDriver", share: 0.336, transport: Transport::Individual, _priv: () });
        vec.push(Mode { index: vec.len(), name: "CarPassenger", share: 0.126, transport: Transport::Individual, _priv: () });
        assert!(vec.len() == COUNT);
        vec
    };
}
