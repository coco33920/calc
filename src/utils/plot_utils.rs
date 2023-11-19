/*
*
*
* **********
* **title **
* *|       *
* *|+      *
* *|+      *
* *|       *
* *--------*
*
*
*
*/
pub fn computes_lines(x: &Vec<f64>, y: &Vec<f64>, start: f64, end: f64, steps: f64) -> () {
    let mut bitmap = vec![vec![' '; 100]; 30];
    let z = x
        .into_iter()
        .zip(y)
        .map(|(x, y)| ((*x * 10.0) as usize, (*y * 10.0) as usize));

    z.for_each(|(x, y)| {
        if x < 100 && y < 30 {
            bitmap[x][y] = '+'
        }
    });

    let first_line = vec!['*'; 102];
    let last_line = vec!['*'; 102];

    for char in first_line {
        print!("{char}");
    }
    println!("");
    for x in (0..(bitmap.len())).rev() {
        print!("*");
        let xs = &bitmap[x];
        for y in 0..xs.len() {
            print!("{}", xs[y]);
        }
        print!("*\n");
    }
    for char in last_line {
        print!("{char}");
    }
    println!("");
}
