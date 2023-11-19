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
pub fn computes_lines(
    x: &Vec<f64>,
    y: &Vec<f64>,
    start: f64,
    end: f64,
    steps: f64,
    title: String,
    xlabel: String,
    ylabel: String,
) -> () {
    let mut bitmap = vec![vec![' '; 100]; 30];
    let z = x
        .into_iter()
        .zip(y)
        .map(|(x, y)| (((*x - start) / steps) as usize, *y as usize));

    z.for_each(|(x, y)| {
        if x < 100 && y < 30 {
            bitmap[y][x] = '+';
        }
    });

    let first_line = vec!['*'; 103];
    let last_line = vec!['*'; 103];

    let x_line = vec!['-'; 100];
    for char in first_line {
        print!("{char}");
    }

    println!("");

    if &title != "" {
        let left_padding = (103 - title.len()) / 2;
        let right_padding = (103 - title.len()) - left_padding;
        for _ in 0..left_padding {
            print!("*");
        }
        print!("{}", title);
        for _ in 0..right_padding {
            print!("*")
        }

        println!("");
    }

    let size = ylabel.len();
    let i = ylabel.chars();
    let mut label = Vec::new();
    let f = 30 / 2 - size;
    let e = 30 - f - size;
    for _ in 0..f {
        label.push('*')
    }
    i.for_each(|x| label.push(x));
    for _ in 0..e {
        label.push('*')
    }
    let mut iter_label = label.into_iter();

    for x in (0..(bitmap.len())).rev() {
        print!("{}", iter_label.next().unwrap());
        print!("|");
        let xs = &bitmap[x];
        for y in 0..xs.len() {
            print!("{}", xs[y]);
        }
        print!("*\n");
    }

    print!("*|");
    for char in x_line {
        print!("{char}");
    }
    println!("*");

    print!("* ");
    let string_start = format!("{start}").len();
    let string_end = format!("{end}").len();
    let middle = ((end + start) / 2.0) as i32;
    let middle_string = format!("{middle}").len();
    print!("{start}");
    for _ in (string_start)..(100 / 2) {
        print!(" ");
    }
    print!("{middle}");
    for _ in (middle_string)..(100 / 2 - 1 - string_end) {
        print!(" ");
    }
    println!("{end} *");

    if &xlabel != "" {
        let first = 103 / 2 - xlabel.len();
        let last = 103 - first - xlabel.len();
        for _ in 0..first {
            print!("*")
        }
        print!("{xlabel}");
        for _ in 0..last {
            print!("*")
        }
    } else {
        for char in last_line {
            print!("{char}");
        }
    }

    println!("");
}
