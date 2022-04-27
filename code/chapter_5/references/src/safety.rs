/** Example of borrowing a local variable error
pub fn borrowing_local() {
    let r;
    {
        let x = 1;
        r = &x;
    }
    // Error porque x deja de existir en el anterior contexto
    // pero r sigue existiendo hasta llegar al final del cuerpo de la
    // funcion
    assert_eq!(*r, 1);
}
*/

/**
 * Example of borrowing a local variable error
 **/
pub fn borrowing_local_working() {
    let r;
    {
        let x = 1;
        r = &x;
        assert_eq!(*r, 1);
    }
}

pub fn g<'a>(p: &'a i32) {
    println!("{}", p);
}

pub fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if r < s {
            s = r;
        }
    }
    s
}

pub struct S<'a> {
    pub r: &'a i32,
}
