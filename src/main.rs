use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

type HouseholdID = usize;
type UnitID = usize;

struct State {
    households: HashMap<HouseholdID, HouseholdParameters>,
    units: HashMap<UnitID, UnitParameters>,
    vacant_units: Vec<UnitID>,
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
}

trait Simulatable {
    fn create_household(&mut self, parameters: HouseholdParameters) -> HouseholdID {}

    fn create_unit(&mut self, parameters: UnitParameters) -> UnitID {}
}

impl Simulatable for Context {}
