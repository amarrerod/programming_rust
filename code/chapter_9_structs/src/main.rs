pub mod broom;
pub mod extrema;
pub mod queue;
use broom::{chop, Broom};
use queue::Queue;

fn main() {
    let hokey = Broom::new("Hokey".to_string());
    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.height, 90);
    assert_eq!(hokey1.health, 100);

    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.height, 90);
    assert_eq!(hokey2.health, 100);

    let mut q = Queue::<u32> {
        older: Vec::<u32>::new(),
        younger: Vec::<u32>::new(),
    };

    q.push(0);
    q.push(1);
    assert_eq!(q.pop(), Some(0));
    q.push(100);
    assert_eq!(q.pop(), Some(100));
    assert_eq!(q.pop(), Some(1));
    assert_eq!(q.pop(), None);
    q.push(100);
    assert_eq!(q.pop().unwrap(), 100);

    let (older, younger) = q.split();
    assert_eq!(older.len(), 0);
    assert_eq!(younger.len(), 0);

    let mut bq = Box::new(Queue::<u32>::new(vec![], vec![]));
    bq.push(10000);

    let mut fq = Queue::<f64> {
        older: vec![1.0; 100],
        younger: vec![2.0; 100],
    };
    println!("{}", fq.sum());

    let a = [0, -3, 0, 15, 48];
    let e = extrema::find_extrema(&a);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);
}
