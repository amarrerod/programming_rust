mod binary_tree;
mod ordering;
mod shape;
mod time;

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
}
