const FREEZINGPOINT:f32 = 32.0;

fn fahrenheit_to_celsius(temperature: f64) -> f64{
    (temperature - FREEZINGPOINT as f64) * (5.0/9.0) as f64
}

fn main(){

println!("{}", FREEZINGPOINT);
let mut fahrenheit: f64 = 32.0;

let in_celsius = fahrenheit_to_celsius(fahrenheit);

println!("{}", in_celsius);

let temps = [33.0, 34.0, 35.0, 36.0, 37.0]; 

for &temp in temps.iter(){

    println! ("{:?}", fahrenheit_to_celsius(temp))
}

}