fn print_padovan() {
    let mut padovan = vec![1, 1, 1];
    for i in 3..50 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..50) = {:?}", padovan);
}

fn allocate_in_heap() {
    let point = Box::new((0.625, 0.5));
    let label = format!("{:?}", point);
    assert_eq!(label, "(0.625, 0.5)");
}

struct Person {
    name: String,
    birth: i32,
}

fn composers() {
    let mut comps = Vec::<Person>::new();
    comps.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    comps.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    comps.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });
    for c in &comps {
        println!("{}, born in {}", c.name, c.birth);
    }
}

fn moves() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let _t = s;
    // No va a funcionar porque el valor de s fue movido a t
    // let u = s;
}

/**
 * Para obtener un resultado similar a C++
 * tenemos que hacer copias invocando al método clone del objeto
 */
fn clones() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s.clone();
    let u = s.clone();
    println!("{:?}, {:?}, {:?}", s, t, u);
}

fn operations_that_move() {
    let mut s = "Govinda".to_string();
    s = "Siddhartha".to_string();
    println!("{}", s);

    let mut x = "Govinda".to_string();
    let t = x; // T takes the ownership of "Govinda"
               // println!("{}", x); --> Error because the value has been borrowed from x
    x = "Siddhartha".to_string();
    println!("{}, {}", x, t);
}

/**
 * Como podemos mover un valor de un vector
 */
fn move_from_vector() {
    let mut v = Vec::<String>::new();
    for i in 100..150 {
        v.push(i.to_string());
    }

    // Extraer el último valor con pop()
    let last_e = v.pop().expect("vector empty!");
    assert_eq!(last_e, "149");

    // 2. Mover un valor del vector en el índice
    // indicado mientras se intercambia
    // el espacio vacío por el último elemento del vector
    let other = v.swap_remove(1);
    assert_eq!(other, "101");

    // 3. Sustituir el valor que sacamos por otro nuevo
    let third = std::mem::replace(&mut v[2], "Substitute".to_string());
    assert_eq!(third, "102");

    println!("{:?}", v);
}

fn loop_vector() {
    let v = vec![
        "Liberté".to_string(),
        "Égalité".to_string(),
        "Fraternité".to_string(),
    ];
    for mut s in &v {
        let mut other = s.clone();
        other.push('!');
        println!("{}", other);
    }
    println!("{:?}", v);
}

fn copy_types() {
    let string1 = "Somnambulance".to_string();
    let string2 = string1;
    let num1: i32 = 36;
    let num2 = num1;
    println!("{}, {}", num1, num2);

    let tuple1 = (0.5, 1.0);
    let tuple2 = tuple1;
    println!("{:?}, {:?}", tuple1, tuple2);
}
/**
 * Incluimos que derive de Copy y Clone para poner hacer copias de las etiquetas
 */
#[derive(Copy, Clone)]
struct Label {
    number: i32,
}

fn print_label(l: Label) {
    println!("STAMP: {}", l.number);
}

fn main() {
    println!("Hello, Rust!");
    print_padovan();
    allocate_in_heap();
    composers();
    moves();
    clones();
    operations_that_move();
    move_from_vector();
    loop_vector();
    copy_types();

    let l = Label { number: 3 };
    print_label(l);
    println!("{}", l.number);
}
