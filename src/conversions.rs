// Convert Celsius to Fahrenheit
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

// Convert hectopascals to inches of mercury
pub fn hpa_to_inhg(hpa: f64) -> f64 {
    hpa * 0.02953
}

// Convert meters per second to miles per hour
pub fn ms_to_mph(ms: f64) -> f64 {
    ms * 2.23694
}

// Convert millimeters to inches
pub fn mm_to_inches(mm: f64) -> f64 {
    mm * 0.0393701
}

// Calculate dewpoint in Fahrenheit using Magnus formula
pub fn calculate_dewpoint_f(temp_c: f64, humidity: f64) -> f64 {
    let a = 17.27;
    let b = 237.7;
    let alpha = ((a * temp_c) / (b + temp_c)) + (humidity / 100.0).ln();
    let dewpoint_c = (b * alpha) / (a - alpha);

    celsius_to_fahrenheit(dewpoint_c)
}
