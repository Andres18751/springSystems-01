const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit (c: f64) -> f64{
    c * 9.0 / 5.0 + FREEZING_POINT_F
}

fn main() { 
    let mut int_temp_f = 32.0;

    let int_c = fahrenheit_to_celsius(int_temp_f);
    println!("{}F = {:2}C", int_temp_f, int_c);

    println!("\nNext 5 integer temperatures:");
      for offset in 1..=5 {
        let f = int_temp_f + offset as f64;
        let c = fahrenheit_to_celsius(f);
        println!("{}F = {:.2}C", f, c)
    }    
}         
