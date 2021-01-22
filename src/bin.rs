use std::env;

use serde_json;        



extern crate simple_lib;
use simulation_state::simulation_state::SimulationState;
use calendar::date::Date;
use simple_test_buildings::*;
use simple_test_people::*;
use weather::epw_weather::EPWWeather;


fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Error... Usage is: {} epw_file", args[0]);
        return;
    }
    
    let weather_file = args[1].clone();

    
    let mut state = SimulationState::new();        
    let person = get_person(&mut state);
    let building = get_single_zone_building_with_heater(&mut state, 20.0);

    let weather = EPWWeather::from_file(weather_file);    

    let start = Date{
        day: 1,
        month: 1,
        hour: 0.0,
    };

    let mut end = start.clone();
    end.add_hours(72.0);// simulate one week

    let n = 12; // tsteps per hour

    let results = simple_lib::run(start, end, &person, &building, &mut state, &weather, n).unwrap();

    println!("{}",serde_json::to_string_pretty(&results).unwrap())
    
    
}
