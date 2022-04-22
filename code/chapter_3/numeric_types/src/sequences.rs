pub fn arrays() -> bool {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);
    // Crear un array con muchos trues
    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    println!("{:?}", sieve);
    true
}

pub fn vectors() -> bool {
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);
    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);
    let new_pixel_buffer = vec![0; 32 * 32];
    println!("{:?}", new_pixel_buffer);

    // Se puede crear un con iterador
    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    // Iterar un vector con un loop
    let languages: Vec<&str> = vec!["English", "Spanish", "Finish", "German", "French"];
    for l in languages {
        println!(
            "{}: {}",
            l,
            if l.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            }
        );
    }
    true
}

pub fn slices() -> bool {
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];

    let slice_v: &[f64] = &v;
    let slice_a: &[f64] = &a;

    true
}
