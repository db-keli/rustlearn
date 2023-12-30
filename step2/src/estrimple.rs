#[derive(Debug)]
pub enum VehicleColor {
    Blue,
    Brown,
    Black,
    Yellow,
    Green,
}
#[derive(Debug)]
pub struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    color: VehicleColor,
}

impl Vehicle {
    fn paint(&mut self, new_color: VehicleColor) {
        self.color = new_color;
    }
}

fn new_vehicle() -> Vehicle {
    let mut v1 = Vehicle {
        manufacturer: "Mercedes Benz".to_string(),
        model: "C-250".to_string(),
        year: 2021,
        color: VehicleColor::Green,
    };
    v1.paint(VehicleColor::Yellow);
    return v1;
}

pub fn create_vehicle() {
    let myvehicle = new_vehicle();
    println!("{:?}", myvehicle);
}