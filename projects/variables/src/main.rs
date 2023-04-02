fn main() {
    let x = 5;
    let x = x + 1;
    
    {
        let x = x * 2;
        println!("x = {x}");
    }
    println!("x = {x}");
    
    let spaces = "xxxxx";
    println!("spaces = {spaces}");
    let spaces = spaces.len();
    println!("spaces = {spaces}");

    let character: char = char::from_u32(0x0e01).expect("ERROR");
    println!("Character = {character}");

    let tuple: (i32, f64, u8) = (500, 6.4, 0xff);
    println!("0 = {}, 1 = {}, 2 = {}", tuple.0, tuple.1, tuple.2);
    let (x, y, z) = tuple;
    println!("x = {x}, y = {y}, z = {z}");

    let a: [u64; 5] = [1, 2, 3, 4, 5];
    println!("{} {} {} {} {}", a[0], a[1], a[2], a[3], a[4]);
    
    let a: [u64; 5] = [3; 5];
    println!("{} {} {} {} {}", a[0], a[1], a[2], a[3], a[4]);

    print_labeled_measurement(5, "metres");

    let y = {
        let x = 3;
        x + 1
    };
    println!("y = {y}");

    let x = five();
    println!("x = {x}");
    
    let x = add_one(x);
    println!("x = {x}");

    if x < 5 {
        println!("True");
    } else {
        println!("False");
    }

    let x_type = if x % 2 == 0 { "even" } else { "odd" };
    println!("x type = {x_type}");
    
    let z = -2;
    let z_type = if z < 0 {
        "zero"
    } else if z % 2 == 0 {
        "even"
    } else {
        "odd"
    };
    println!("z type = {z_type}");

    let mut i = 0;
    let result = loop {
        i += 1;
        if i > 9 {
            break i * 10;
        }
    };
    println!("result = {result}");

    let mut i = 0;
    'vertical: loop {
        print!("{i} ");

        let mut j = 0;
        'horizontal: loop {
            if i > 9 {
                println!("");
                break 'vertical;
            }
            if j > i {
                break 'horizontal;
            }
            
            print!("*");
            j += 1;
        }
        println!("");

        i += 1;
    }

    while i > 0 {
        i -= 1;
        println!("{i}");
    }

    let arr: [u32; 11] = [0x00, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xa0];
    for num in arr {
        println!("num: {num}");
    }

    for number in 1..11 {
        println!("{number}");
    }
    for number in (1..11).rev() {
        println!("{number}");
    }
}

fn print_labeled_measurement(value: i64, unit_label: &str) {
    println!("The measurement is {value} {unit_label}");
}

fn five() -> i32 {
    5
}

fn add_one(x: i32) -> i32 {
    x + 1
}
