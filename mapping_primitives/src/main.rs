
fn main() {
    let lat1: f64 = 51.5074;
    let lon1: f64 = 0.1278;
    let lat2: f64 = 48.8566;
    let lon2: f64 = 2.3522;
    let R: f64 = 6371e3; // metres
    let φ1: f64 = (lat1 * std::f64::consts::PI) / 180.; // φ, λ in radians
    let φ2: f64 = (lat2 * std::f64::consts::PI) / 180.;
    let Δφ: f64 = ((lat2 - lat1) * std::f64::consts::PI) / 180.;
    let Δλ: f64 = ((lon2 - lon1) * std::f64::consts::PI) / 180.;

    let a =
        (Δφ / 2.).sin() * (Δφ / 2.).sin() +
        φ1.cos() * φ2.cos() * (Δλ / 2.).sin() * (Δλ / 2.).sin();
    let c: f64 = 2. * a.sqrt().atan2((1. - a).sqrt());

    let d: f64 = R * c; // in metres
    println!("{}", d);
}
