use rand::seq::index;

mod test;

fn main() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b + c as f64) / 3.0;

    assert_eq!(average, 45.1);
    println!("Test passed!");

    let result = square(12);
    println!("result is {:?}", result);

    let fah = celsium_to_fahrenfeit(20.0);
    println!("Temperature in fahrenfeit is {:?}", fah);

    test_loop();
    iterate_possible();
    find_max_min();
    ownership();

    let mut rocket_fuel = String::from("RP-1");
    let length = borrow(&mut rocket_fuel);
    println!("the length is {}", length);

    chanleng_trim_space();
}

fn square(x: i32) -> (i32, i32) {
    println!("squaring {}", x);
    return (x, x * x);
}

fn celsium_to_fahrenfeit(x: f32) -> f32 {
    x * 1.8 + 32.0
}

fn test_loop() {
    let mut i = 0;
    let result = loop {
        if i == 10 {
            break i * 10;
        }
        i += 1;
        println!("i is {}", i);
    };
    println!("result is {}", result);
}

fn iterate_possible() {
    let mes = ['h', 'e', 'l', 'l', 'o', 't'];
    for (index, &item) in mes.iter().enumerate() {
        if item == 't' {
            break;
        }
        print!("{}", mes[index]);
    }
    for i in 0..5 {
        println!();
        print!("{}", i);
    }
    println!();

    let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    for row in matrix.iter_mut() {
        for item in row.iter_mut() {
            *item += 10;
            print!("{}\t", item);
        }
        println!();
    }
}

fn find_max_min() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = 0;
    let mut min: i32 = 0;
    let mut mean: f64;
    let mut all: i32 = 0;

    for &i in numbers.iter() {
        if i < min {
            min = i;
        } else if i > max {
            max = i;
        }
        all += i;
    }
    let size = numbers.len() as f64;
    mean = all as f64 / size;
    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed")
}

fn ownership() {
    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        outer_planet = inner_planet.clone();
        inner_planet.clear();
        println!("inner_planet is {}", inner_planet);
    }
    println!("outer_planet is {}", outer_planet);
}

fn borrow(propellant: &mut String) -> usize {
    println!("processing propellant is ... {}", propellant);
    propellant.push_str(" is highly flammed!");
    let length = propellant.len();
    length
}

fn chanleng_trim_space() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");

    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");

    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

fn trim_spaces(s: &str) -> &str {
    // locate the first non-space character
    let mut start = 0;
    for (index, character) in s.chars().enumerate() {
        if character != ' ' {
            start = index;
            break;
        }
    }

    // search in reverse to locate the last non-space character
    let mut end = 0;
    for (index, character) in s.chars().rev().enumerate() {
        if character != ' ' {
            end = s.len() - index;
            break;
        }
    }

    &s[start..end]
}
