pub fn move_vector() {
    let v = vec![4, 8, 19, 27, 34, 10];
    {
        let r = &v;
        println!("r is {:?}", r[0]);
        println!("V is: {:?}", v);
    }
    let aside = v;
    println!("v is {:?}", aside);
}
