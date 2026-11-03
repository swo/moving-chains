use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

type HouseholdID = usize;
type UnitID = usize;

struct State {
    // point from household ID to the *hash* of the parameters, then look up the parameters
    // separately
    households: HashMap<HouseholdID, HouseholdParameters>,
    units: HashMap<UnitID, UnitParameters>,
    // not a vector, but a heap or something?
    vacant_units: Vec<UnitID>,
}

impl State {
    fn get_household_parameters(&self, household: HouseholdID) -> &HouseholdParameters {
        panic!("not implemented");
    }
    
    fn get_unit_parameters(&self, household: HouseholdID) -> &HouseholdParameters {
        panic!("not implemented");
    }

    fn get_preference(&self, household: HouseholdID, unit: UnitID) -> f64 {
        panic!("not implemented");
    }

    fn draw_lifetime(&self, household: HouseholdID) -> f64 {
        let parameters = self.get_household_parameters(household);
        panic!("not implemented");
    }

    fn draw_tenure(&self, household: HouseholdID) -> f64 {
        let parameters = self.get_household_parameters(household);
        panic!("not implemented");
    }

    fn get_vacant_units(&self) -> &Vec<UnitID> {
        panic!("not implemented");
    }
}

#[derive(Hash)]
struct HouseholdParameters {
    class: usize,
}

#[derive(Hash)]
struct UnitParameters {
    class: usize,
}

struct Context {
    state: State,
    // need a ::new() or ::init() to initialize this?
    // where do I feed in parameters?
    events: queue??<Event>,
    rng: ??,
    max_time: f64, // or is this a parameter?
}

struct Event {
    time: f64,
    callback: ??
}

impl Context {
    fn create_household(&mut self, parameters: HouseholdParameters) -> HouseholdID {
        // draw their class, as weighted multinomial, based on parameters
        // instantiate them in the state
        // schedule removal
        // schedule first move
    }

    fn create_unit(&mut self, parameters: UnitParameters) -> UnitID {
        // instantiate them in the state
        // that's it?
    }

    fn schedule(&mut self, event: Event) {
        // add it to the event queue
    }

    fn run(&mut self) {
        // while there are events in the queue, proceed
        // unless it's past the max_time, then stop
    }

    fn schedule_removal(&mut self, household: HouseholdID) {
        // draw a household lifetime
        // schedule handle_removal
    }

    fn schedule_move(&mut self, household: HouseholdID) {
        // draw tenure
        // handle_move after the tenure
    }

    fn handle_move(&mut self, household: HouseholdID) {
        // if household has been removed, do nothing
        // select a unit at random, from among vacant units, weighted by preference
        // schedule the next move
    }

    fn handle_removal(&mut self, household: HouseholdID) {
        // if already removed, panic
        // remove them from the heap
    }
}

