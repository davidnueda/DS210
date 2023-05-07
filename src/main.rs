mod preprocessing;
mod logistic_regression;
mod graph;

use preprocessing::{preprocess_data, read_data};
use logistic_regression::logistic_regression;
use logistic_regression::{make_prediction};
use graph::create_scatter_plot;

use std::path::Path;

fn main() {
    // Preprocess train and test data
    let train_input_path = Path::new("data/train.csv");
    let train_output_path = Path::new("titanic_train_preprocessed.csv");
    if let Err(err) = preprocess_data(&train_input_path, &train_output_path) {
        eprintln!("Error preprocessing train data: {}", err);
    } else {
        println!("Preprocessed train data saved to {:?}", train_output_path);
}

let test_input_path = Path::new("data/test.csv");
let test_output_path = Path::new("titanic_test_preprocessed.csv");
if let Err(err) = preprocess_data(&test_input_path, &test_output_path) {
    eprintln!("Error preprocessing test data: {}", err);
} else {
    println!("Preprocessed test data saved to {:?}", test_output_path);
}


    // Train the logistic regression model using the preprocessed training data
    let train_filename = "titanic_train_preprocessed.csv";
    let train_passengers = read_data(train_filename);
    let learning_rate = 0.01;
    let num_iterations = 1000;
    let weights = logistic_regression(&train_passengers, learning_rate, num_iterations);

println!("Weights: {:?}", weights);
let test_filename = "titanic_test_preprocessed.csv";
let passengers_test = read_data(test_filename);
let predictions: Vec<f64> = passengers_test
    .iter()
    .map(|passenger| make_prediction(&weights, passenger))
    .collect();

    let output_file = "scatter_plot.png";
    let x_label = "Pclass";
    let y_label = "Age";
    if let Err(e) = create_scatter_plot(&train_passengers, output_file, x_label, y_label) {
        eprintln!("Error creating scatter plot: {}", e);
    } else {
        println!("Scatter plot saved to {}", output_file);
    }
}



