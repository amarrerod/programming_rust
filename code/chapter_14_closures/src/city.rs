#[derive(Debug, Default)]
pub struct City {
    name: String,
    population: i64,
    country: String,
    pub monster_attacks: i32,
}

impl City {
    pub fn new(name: String, population: i64, country: String) -> City {
        City {
            name: name,
            population: population,
            country: country,
            ..Default::default()
        }
    }
}

fn city_population_descending(city: &City) -> i64 {
    -city.population
}

pub fn sort_cities(cities: &mut Vec<City>) {
    // cities.sort_by_key(city_population_descending);
    // Better approach using closures
    cities.sort_by_key(|city| -city.population);
}

/// Given a vector of cities and a test functio,
/// returns how many cities passed the test
pub fn count_selected_cities<F>(cities: &Vec<City>, test_fn: F) -> usize
where
    F: Fn(&City) -> bool,
{
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }
    count
}
