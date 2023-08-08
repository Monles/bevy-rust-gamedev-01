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
    Analyst,
    PromptEngineer,
    Lawyer,
    Painter,
    GameDeveloper,
    Animator,
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Alice".to_string(),
        },
        Employed {
            job: Job::Analyst,
        },
    ));
    commands.spawn((
        Person {
            name: "Bob".to_string(),
        },
        Employed {
            job: Job::Animator,
        },
    ));
    commands.spawn((
        Person {
            name: "Catherine".to_string(),
        },
        Employed {
            job: Job::Painter,
        },
    ));
    commands.spawn((
        Person {
            name: "Dave".to_string(),
        },
        Employed {
            job: Job::Doctor,
        },
    ));
    commands.spawn((
        Person {
            name: "Elliotte".to_string(),
        },
        Employed {
            job: Job::GameDeveloper,
        },
    ));
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{} has a job.", person.name);
    }
}

pub fn people_ready_for_hire(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("{} is ready for hire.", person.name);
    }
}

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, (print_names, people_with_jobs))
        .run();
}
