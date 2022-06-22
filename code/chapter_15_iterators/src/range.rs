pub struct I32Range {
    pub start: i32,
    pub end: i32,
}

// Implementation of Iterator trait for the I32Range struct
// La librería estándar provee de IntoIterator para cada tipo
// que implementa Iterator
impl Iterator for I32Range {
    type Item = i32;
    // Tiene que devolver Option<i32> porque el tipo es i32
    // y además Option para que cuando lleguemos al final devolvamos el None
    fn next(&mut self) -> Option<i32> {
        if self.start >= self.end {
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}
