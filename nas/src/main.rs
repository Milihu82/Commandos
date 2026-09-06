fn main() {
    //Posiciones de la pantalla
    let left: u64 = 0;
    let right: u64 = 80;
    let midle: u64 = 40;

    //Imagenes
    let pajaro: String = "<o|-<".to_string();

    //Main loop
    for frame in 40 {
        let spaces: String = " ".repeat(80-frame);
        println!("{}{}", spaces, pajaro);
    }
}
