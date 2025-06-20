mod wgs84;
mod ecef;
use wgs84::WGS84;
use ecef::ECEF;

use std::f64::consts::PI;

fn main() {
   
    // top of eiffel tower
    let radar = WGS84 {         
        latitude: 48.8584,
        longitude: 2.2945,
        elevation: 330.0,
    };

    // example coordinates used in these MatLab docs: https://www.mathworks.com/help/map/choose-a-3-d-coordinate-system.html
    let satellite = ECEF {      
        x: 4198945.0,  
        y: 174747.0,
        z: 4781887.0,
    };

    let range = calculate_range(radar, satellite);

    println!("The distance between the radar and the satellite is {} meters", range)
}

/**
 * I'm assuming I shouldn't use an existing library to calculate the coordinate conversions for me, 
 * but I did find and look into the nav_types crate but chose not to use it.
 * 
 * 
 * calculates the distance betwen a radar in WGS84 coordinates and a satellite in ECEF coordinates
 */
pub fn calculate_range(radar_position: WGS84, satellite_position: ECEF) -> f64 {
    
    // getting ecef coords for radar
    let radar_ecef : ECEF = convert_wgs84_ecef(radar_position);
    
    // getting deltas of each coordinate
    let delta_x : f64 = satellite_position.x - radar_ecef.x;
    let delta_y : f64 = satellite_position.y - radar_ecef.y;
    let delta_z : f64 = satellite_position.z - radar_ecef.z;

    // using 3D pythagorean theorem to calculate final range
    let x_sq : f64 = delta_x * delta_x;
    let y_sq : f64 = delta_y * delta_y;
    let z_sq : f64 = delta_z * delta_z;
    return (x_sq + y_sq + z_sq).sqrt();
}


/**
 * takes in WGS84 coordinates and converts them to ECEF coordinates
 */
pub fn convert_wgs84_ecef(wgs84 : WGS84) -> ECEF {

    // used formulas from this wikipedia article https://en.wikipedia.org/wiki/Geographic_coordinate_conversion#From_geodetic_to_ECEF_coordinates

    // earth's axis' in meters according to https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html
    let semi_major_axis : f64 = 6378137.0;         
    let semi_minor_axis : f64 = 6356752.0;

    // calcualting eccentricity
    let eccentricity : f64 = (1.0 - ((semi_minor_axis * semi_minor_axis)/(semi_major_axis * semi_major_axis))).sqrt();
    let e_squared : f64 = eccentricity * eccentricity;

    // sine/cosine values of latitude and longitude
    let latitude_radians : f64 = degrees_to_radians(&wgs84.latitude);
    let longitude_radians : f64 = degrees_to_radians(&wgs84.longitude);
    let sin_latitude : f64 = latitude_radians.sin();
    let sin_longitude : f64 = longitude_radians.sin();
    let cos_latitude : f64 = latitude_radians.cos();
    let cos_longitude : f64 = longitude_radians.cos();

    // calculating prime vertical 
    let prime_vertical : f64 = semi_major_axis / (1.0 - e_squared * sin_latitude * sin_latitude).sqrt();

    // calculating x
    let x : f64 = (prime_vertical + wgs84.elevation) * cos_latitude * cos_longitude;

    // calculating y
    let y : f64 = (prime_vertical + wgs84.elevation) * cos_latitude * sin_longitude;

    // calcualting z
    let z : f64 = ((1.0 - e_squared) * prime_vertical + wgs84.elevation) * sin_latitude;

    return ECEF {x, y, z};
}


/**
 * converts to degrees to radians
 */
fn degrees_to_radians(degrees: &f64) -> f64 {
    degrees * PI / 180.0
}
