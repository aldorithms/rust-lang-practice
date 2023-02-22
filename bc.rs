use std::collections::HashMap;
use std::ops::Add;
use serde_json::{Serialize, Deserialize};

#[derive(Debug)]
struct Config<'src>
{
    hostname: &'src str,
    username: &'src str,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point
{
    x: i32,
    y: i32,
}

fn factorial(i: u64) -> u64 {
    if i == 0 {
        1
    } else {
        i * factorial(i - 1)
    }
}

fn sum<T>(num1: T, num2: T) -> T where T: Add<Output = T>,
{
    num1 + num2
}

fn parse_config<'cfg>(config: &'cfg str) -> Config<'cfg>
{
    let key_values: HashMap<_, _> = config.lines().filter(|line| !line.starts_with('#')).filter_map(|line| line.split_once('=')).map(|(key, value)| (key.trim(), value.trim())).collect();
    Config
    {
        hostname: key_values["hostname"],
        username: key_values["username"],
    }
}

fn main()
{
    println!("Hello, World!");
    let mut values = vec![1, 2, 3, 4];

    for value in &values
    {
        println!("value = {}", value);
    }

    if values.len() > 5
    {
        println!("List is longer than five items");
    }

    match values.len()
    {
        0 => println!("Empty"),
        1 => println!("One value"),
        2..=10 => println!("Between two and ten values"),
        11 => println!("Eleven values"),
        _ => println!("Many values"),
    };

    let result1 = sum(10, 20);
    println!("Sum is: {}", result1);

    let result2 = sum(10.23, 20.45);
    println!("Sum is: {}", result2);

    while let Some(value) = values.pop()
    {
        println!("value = {value}");
    }

    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    let name: Option<String> = None;

    if let Some(name) = name {
        println!("{}", name);
    }

    let config = parse_config(r#"hostname = foobar username=barfoo"#,);
    println!("Parsed config: {:#?}", config);
}

