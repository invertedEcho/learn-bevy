use bevy::prelude::*;

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PeoplePlugin)
        .run();
}

pub struct PeoplePlugin;

// Implement the Plugin trait
impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, print_names)
            .add_systems(Update, people_with_jobs)
            .add_systems(Update, people_ready_for_hire)
            .add_systems(Update, person_does_job);
    }
}

pub fn setup(mut commands: Commands) {
    // Create an entity
    commands.spawn((
        Person {
            name: "Jakob Stechow".to_string(),
        },
        Employed { job: Job::Lawyer },
    ));
    commands.spawn(Person {
        name: "Max Mustermann".to_string(),
    });
}

// This is how we write basic queries.
pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

// This is how we write a query with a filter. The With filter says: We only want persons that
// have the Employed Component
pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{} has a Job", person.name)
    }
}

// This is how we write a query with a filter. The Without  filter says: We only want persons that
// dont have the Employed Component
pub fn people_ready_for_hire(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("{} is ready to be hired", person.name);
    }
}

// If we want to work on multiple Components, we need to use a tuple in our Query struct,
// containing all the Components we want to work on.
pub fn person_does_job(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query.iter() {
        println!("Person {} has job: {:?}", person.name, employed.job);
    }
}
