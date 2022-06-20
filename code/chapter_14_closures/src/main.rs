mod city;
mod router;

fn has_monster_attacks(city: &city::City) -> bool {
    city.monster_attacks > 0
}

fn call_twice<F>(mut closure: F)
where
    F: FnMut(),
{
    closure();
    closure();
}

fn get_form_response() -> router::Response {
    router::Response::new(32)
}

fn main() {
    let mut cities = vec![
        city::City::new("Madrid".to_string(), 1e5 as i64, "Spain".to_string()),
        city::City::new("Barcelona".to_string(), 3e5 as i64, "Spain".to_string()),
        city::City::new("Tenerife".to_string(), 1e3 as i64, "Spain".to_string()),
    ];

    city::sort_cities(&mut cities);
    println!("{:?}", cities);
    let n = city::count_selected_cities(&cities, has_monster_attacks);
    println!("{:?}", n);

    let limit = 10;
    let n = city::count_selected_cities(&cities, |city| city.monster_attacks > limit);

    let mut i = 0;
    let incr = || {
        i += 1;
        println!("Ding! i is now: {}", i);
    };
    call_twice(incr);

    let mut router = router::BasicRouter::new();
    router.add_route("/", |_| get_form_response());
}
