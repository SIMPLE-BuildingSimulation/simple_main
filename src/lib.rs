use building_model::building_state::BuildingState;
use building_model::building::Building;
use communication_traits::simulation_model::SimulationModel;

use calendar::date_factory::DateFactory;
use calendar::date::Date;


use people::people::People;

use multiphysics_model::multiphysics_model::MultiphysicsModel;

use weather::Weather;


/// This function drives the simulation, after having parsed and built
/// the Building, State and Peoeple.
pub fn run(start: Date, end: Date, person: &dyn People, building: &Building, state: &mut BuildingState, weather: &dyn Weather, n: usize)->Result<(),String>{
    
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

    // Simulate the whole simulation perod
    for date in sim_period {    
        
        // Print results
        state.print_values();
        println!("==");
        
        // Get the current weather data
        let current_weather = weather.get_weather_data(date);

        // Make the model march
        match model.march(building, state, &current_weather ) {
            Ok(_)=>{},
            Err(e) => panic!(e)
        }

        let dwelling_satisfaction = person.calculate_building_immediate_satisfaction(&building, &model, &state);

        // Think about all this ONLY if the person attends
        // the situation
        if person.attend(date, dwelling_satisfaction) {
            
            // calculate current comfort, considering the future...
            // So, clone the state.
            let mut current_state_clone = state.clone();
            // Perform the simulation
            let (current_comfort, current_status) = person.forsee(date, building, &model, &mut current_state_clone);                   
            // Reset the physics
            current_state_clone.copy_physical_state_from(&state);

            // Infer potential comfort
            let (potential_comfort, potential_state) = person.infer_expected_outcomes(date, &building, &model, current_state_clone, current_comfort, &current_status);

            // Behave (i.e. change the building state) if worth it.
            person.behave_if_worth_it(date, current_comfort, potential_comfort, state, &potential_state);


        }// end of if attend()

    }
    
    Ok(())
    
}




/***********/
/* TESTING */
/***********/




#[cfg(test)]
mod testing{
    use super::*;
    
    use geometry3d::polygon3d::Polygon3D;
    use geometry3d::point3d::Point3D;
    use geometry3d::loop3d::Loop3D;


    use building_model::building::Building;
    use building_model::substance::{SubstanceProperties};
    use building_model::material::{MaterialProperties};
    use building_model::boundary::Boundary;
    use building_model::fenestration::*;
    
    use building_model::heating_cooling::HeatingCoolingKind;
    
        
    use weather::synthetic_weather::SyntheticWeather;

    use schedule::constant::ScheduleConstant;

    use calendar::date::Date;

    use people::person::Person;
    use people::perceptions::Perception;


    // A single space building with one operable window (it is opaque
    // at the moment, but I don't think it matters) and a 1500W
    // heater. The zone has only one Surface.
    fn get_single_zone_building(state: &mut BuildingState)-> Building {

        let mut building = Building::new("The Building".to_string()); 

        // Add the space
        let zone_volume = 40.;
        let space_index = building.add_space("Some space".to_string());
        building.set_space_volume(space_index,zone_volume).unwrap();

        building.add_heating_cooling_to_space(state, space_index, HeatingCoolingKind::IdealHeaterCooler).unwrap();
        building.set_space_max_heating_power(space_index, 1500.).unwrap();

        // Add substance
        let poly_index = building.add_substance("polyurethane".to_string());
        building.set_substance_properties(poly_index, SubstanceProperties{
            thermal_conductivity: 0.0252, // W/m.K            
            specific_heat_capacity: 2400., // J/kg.K
            density: 17.5, // kg/m3... reverse engineered from paper
        }).unwrap();

        // add material
        let mat_index = building.add_material("20mm Poly".to_string());
        building.set_material_properties(mat_index, MaterialProperties{
            thickness: 20./1000.
        }).unwrap();
        building.set_material_substance(mat_index,poly_index).unwrap();

        // Add construction
        let c_index = building.add_construction("The construction".to_string());
        building.add_material_to_construction(c_index, mat_index).unwrap();


        // Create surface geometry
        // Geometry
        let mut the_loop = Loop3D::new();
        let l = 4. as f64;
        the_loop.push( Point3D::new(-l, -l, 0.)).unwrap();
        the_loop.push( Point3D::new(l, -l, 0.)).unwrap();
        the_loop.push( Point3D::new(l, l, 0.)).unwrap();
        the_loop.push( Point3D::new(-l, l, 0.)).unwrap();
        the_loop.close().unwrap();
        
        let mut p = Polygon3D::new(the_loop).unwrap();


        let mut the_inner_loop = Loop3D::new();
        let l = 1. as f64;
        the_inner_loop.push( Point3D::new(-l, -l, 0.)).unwrap();
        the_inner_loop.push( Point3D::new(l, -l, 0.)).unwrap();
        the_inner_loop.push( Point3D::new(l, l, 0.)).unwrap();
        the_inner_loop.push( Point3D::new(-l, l, 0.)).unwrap();
        the_inner_loop.close().unwrap();
        p.cut_hole(the_inner_loop.clone()).unwrap();

        

        // Add surface
        let surface_index = building.add_surface("Surface".to_string());
        building.set_surface_construction(surface_index,c_index).unwrap();
        building.set_surface_polygon(surface_index, p).unwrap();
        
        building.set_surface_front_boundary(surface_index, Boundary::Space(space_index)).unwrap();

        // Add window.        
        let window_polygon = Polygon3D::new(the_inner_loop).unwrap();
        let window_index = building.add_fenestration(state, "Window One".to_string(), OperationType::Binary, FenestrationType::Window);
        building.set_fenestration_construction(window_index, c_index).unwrap();     
        building.set_fenestration_polygon(window_index, window_polygon).unwrap();

        

        return building;

    }

    fn get_person()->Person {
        let mut person = Person::new();
        
        person.set_proactivity(Box::new(ScheduleConstant::new(0.5))).unwrap();
        person.set_busyness(Box::new(ScheduleConstant::new(0.5))).unwrap();
        person.set_awareness_of_the_future(Box::new(ScheduleConstant::new(1.))).unwrap();

        person.add_perception(0.2, Perception::ThermalSensationHot);
        person.add_perception(0.2, Perception::ThermalSensationCold);

        return person;
    }


    #[test]
    fn test_run(){
        
        let mut state = BuildingState::new();        
        let building = get_single_zone_building(&mut state);
        let person = get_person();

        let mut weather = SyntheticWeather::new();
        weather.dry_bulb_temperature = Box::new(ScheduleConstant::new(23.));

        let start = Date{
            day: 1,
            month: 1,
            hour: 0.0,
        };

        let mut end = start.clone();
        end.add_days(7);// simulate one week

        let n = 1; // tsteps per hour

        run(start, end, &person, &building, &mut state, &weather, n).unwrap();


    }
}