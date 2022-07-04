use std::io;

fn main() {
    println!("Lütfen kelvin dercesini giriniz");

    let mut kelvin = String::new();

    io::stdin()
        .read_line(&mut kelvin)
        .expect("Hata kelvin derecesi algılanmadı");

    let kelvin = kelvin.trim();
    let kelvin: f32 = kelvin.parse().unwrap();
    let celsius: f32 = kelvin - 273.0;

    let fahrenheit: f32 = celsius * 1.8 + 32.0;

    println!("Celsius {} fhrenheit {}", { celsius }, fahrenheit.floor())
}
