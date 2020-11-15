use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub fn read_file(filepath: &str) -> String {
    let file = File::open(filepath).expect("could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    buffered_reader
        .read_to_string(&mut contents)
        .expect("could not read file into the string");
    contents
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct EffOrder {
    pub total_order_price: f64,
    pub total_order_flour_consumption: f64,
    pub total_order_efficiency: f64,
}

impl EffOrder {
    pub fn new() -> EffOrder {
        EffOrder {
            total_order_price: 0.0,
            total_order_flour_consumption: 0.0,
            total_order_efficiency: 0.0,
        }
    }
}

use serde_json::Value;
fn main() {
    let data = read_file("src/pastry_orders.json");
    let v: Value = serde_json::from_str(data.as_str()).unwrap();

    let mut unsorted_orders = vec![];
    for order in v.as_array().unwrap() {
        let mut total_order_price = 0.0;
        let mut total_order_flour_consumption = 0.0;
        for item in order.as_array().unwrap() {
            total_order_price +=
                item["price"].as_f64().unwrap() * item["quantity"].as_f64().unwrap();
            total_order_flour_consumption +=
                item["flourConsumption"].as_f64().unwrap() * item["quantity"].as_f64().unwrap();
        }
        let total_order_efficiency = total_order_price / total_order_flour_consumption;
        unsorted_orders.push(EffOrder {
            total_order_price,
            total_order_flour_consumption,
            total_order_efficiency,
        });
    }

    unsorted_orders.retain(|x| !x.total_order_efficiency.is_nan());

    unsorted_orders.sort_by(|a, b| {
        b.total_order_efficiency
            .partial_cmp(&a.total_order_efficiency)
            .unwrap()
    });

    let sorted_orders = unsorted_orders;

    let max_flour_consumption = 60000.00;
    let mut current_flour_consumption = 0.0;
    let mut retained_orders = vec![];

    for order in sorted_orders {
        current_flour_consumption = if current_flour_consumption
            + order.total_order_flour_consumption
            <= max_flour_consumption
        {
            retained_orders.push(EffOrder { ..order });
            current_flour_consumption + order.total_order_flour_consumption
        } else {
            current_flour_consumption
        };
    }

    let mut total_price = 0.0;
    for order in &retained_orders {
        total_price += order.total_order_price;
        println!("{:?}", order)
    }

    println!(
        "orders retained: {}, total price: {}, leftover flour: {}",
        retained_orders.len(),
        total_price,
        max_flour_consumption - current_flour_consumption
    )
}
