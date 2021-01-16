use simulation_state::simulation_state::SimulationState;
use building_model::building::Building;
use communication_protocols::simulation_model::SimulationModel;
//use controller::Controller;

use calendar::date_factory::DateFactory;
use calendar::date::Date;


use people::people::People;

use multiphysics_model::multiphysics_model::MultiphysicsModel;

use weather::Weather;


/// This function drives the simulation, after having parsed and built
/// the Building, State and Peoeple.
pub fn run(start: Date, end: Date, person: &dyn People, building: &Building, state: &mut SimulationState, weather: &dyn Weather, n: usize)->Result<(),String>{
    
    if start.is_equal(end) || start.is_later(end) {
        return Err(format!("Time period inconsistency... Start = {} | End = {}", start, end));
    }

    let model = match MultiphysicsModel::new(&building, state, n){
        Ok(v)=>v,
        Err(e)=>return Err(e),
    };    


    // Calculate timestep and build the Simulation Period
    let dt = 60. * 60. / n as f64;
    let sim_period = DateFactory::new(start, end, dt);

    state.print_header();

    // Simulate the whole simulation period
    for date in sim_period {    
            
        // Get the current weather data
        //let current_weather = weather.get_weather_data(date);

        // Make the model march
        match model.march(date, weather, building, state ) {
            Ok(_)=>{},
            Err(e) => panic!(e)
        }

        // Control the building or person, if needed        
        if person.control(date, weather, building, &model, state) {
            println!("Person did something!");
        }
        println!("======")
        

    }
    
    Ok(())
    
}




/***********/
/* TESTING */
/***********/




#[cfg(test)]
mod testing{
    use super::*;
    
    
    
        
    use weather::synthetic_weather::SyntheticWeather;
    use schedule::constant::ScheduleConstant;
    use calendar::date::Date;


    use simple_test_buildings::*;
    use simple_test_people::*;
    


    #[test]
    fn test_run(){
        
        let mut state = SimulationState::new();        
        let person = get_person(&mut state);
        let building = get_single_zone_building(&mut state);

        let mut weather = SyntheticWeather::new();
        weather.dry_bulb_temperature = Box::new(ScheduleConstant::new(222.));

        let start = Date{
            day: 1,
            month: 1,
            hour: 0.0,
        };

        let mut end = start.clone();
        end.add_hours(0.5);// simulate one week

        let n = 12; // tsteps per hour

        run(start, end, &person, &building, &mut state, &weather, n).unwrap();


    }
}