mod binary_tree;
mod ordering;
mod patterns;
mod shape;
mod time;

pub use shape::*;

use binary_tree::*;

fn create_tree() {
    let jupyter_tree = NonEmpty(Box::new(TreeNode {
        element: "Jupyter",
        left: Empty,
        right: Empty,
    }));
    let mercury_tree = NonEmpty(Box::new(TreeNode {
        element: "Mercury",
        left: Empty,
        right: Empty,
    }));
    let mars_tree = NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupyter_tree,
        right: mercury_tree,
    }));
    let venus_tree = NonEmpty(Box::new(TreeNode {
        element: "Venus",
        left: Empty,
        right: Empty,
    }));
    let uranus_tree = NonEmpty(Box::new(TreeNode {
        element: "Uranus",
        left: Empty,
        right: venus_tree,
    }));
    let saturn_tree = NonEmpty(Box::new(TreeNode {
        element: "Saturn",
        left: mars_tree,
        right: uranus_tree,
    }));
    println!("{:#?}", saturn_tree);
}

fn main() {
    println!("Hello, world!");
    let result = ordering::compare(1, 2);
    println!("{:?}", result);

    let time: time::TimeUnit = time::TimeUnit::Seconds;
    let plural = time.plural();
    let singular = time.singular();
    println!("Singular is: {:?}, Plural is: {:?}", singular, plural);

    let four_score_and_seven_years_ago =
        time::RoughTime::InThePast(time::TimeUnit::Years, 4 * 20 + 7);
    let three_hours_from_now = time::RoughTime::InTheFuture(time::TimeUnit::Hours, 3);
    println!(
        "{:?}, {:?}",
        four_score_and_seven_years_ago, three_hours_from_now
    );

    let unit_sphere = shape::Shape::create_unit_sphere();
    println!("The Unit Sphere is: {:#?}", unit_sphere);
    create_tree();
    patterns::number_of_rabbits(0);
    patterns::number_of_rabbits(1);
    patterns::number_of_rabbits(100);

    let gre_calendar = patterns::calendar_type("gregorian").unwrap();
    let chi_calendar = patterns::calendar_type("chinese").unwrap();
    println!("The calendar is: {:?}", gre_calendar);
    println!("The chi calendar is: {:?}", chi_calendar);

    match patterns::calendar_type("none") {
        None => println!("No calendar specified"),
        _ => println!("There is a calendar specified"),
    };

    println!("{}", patterns::describe_point(0, 0));
    println!("{}", patterns::describe_point(1, 0));
    println!("{}", patterns::describe_point(0, 1));
    println!("{}", patterns::describe_point(100, 011));
    println!("{}", patterns::describe_point(-100, 011));

    println!("{}", patterns::describe_point(110, -1110));

    let point = shape::ORIGIN;
    let other = shape::Point3d::new(0.0, 1.0, 2.0);
    patterns::balloon_location(point);
    patterns::balloon_location(other);
    let names: [&str; 7] = [
        "Leonard",
        "Rajesh",
        "Sheldon",
        "Howard",
        "Penny",
        "Amy",
        "Bernadette",
    ];
    patterns::greet_peopple(&names);
    patterns::greet_peopple(&names[..0]);
    patterns::greet_peopple(&names[..1]);
    patterns::greet_peopple(&names[..2]);
    patterns::greet_peopple(&names[..3]);
}
