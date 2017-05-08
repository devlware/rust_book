fn main() {
    // loops forever
    //loop {
    //    println!("Hello, world!");
    //}

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // using a while to behave like a for which is not optimal.
    println!("using a while loop to print contents of an array");
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    println!("using a for loop to print the contents of an array");
    let b = [2,3,4,5,5,6];
    for element in b.iter() {
        println!("the value is: {}", element);
    }

    println!("using for + range + rev()");
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");

    for x in (0..10).rev() {
        println!("{}", x);
    }

    let temp = 27.45;
    print_in_farad(temp);
}

fn print_in_farad(t: f32) {
    let temp_far = (t  * 1.8) + 32.0;
    println!("temp {} in Celcius is {} in Farenheit.", t,  temp_far);
}

// Celsius	Fahrenheit	°F = °C × 1.8 + 32
// Fahrenheit	Celsius	°C = (°F – 32) / 1.8
// Celsius	kelvin	K = °C + 273.15
// kelvin	Celsius	°C = K – 273.15
