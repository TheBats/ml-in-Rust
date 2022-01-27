use std::time::Instant;

mod dataframe;
mod data_preparation;
mod regression;

use dataframe::DataFrame;

fn main() {

    let start = Instant::now();
    let data = DataFrame::from_path("src/housing.csv").unwrap();
    println!("Elapsed loading data: {:?}", start.elapsed());

    let start = Instant::now();

    let median_age = data.to::<f64>("housing_median_age");
    let median_house_value = data.to::<f64>("median_house_value");
    
    println!("Elapsed getting as f64: {:?}", start.elapsed());

    let (train_data, train_labels, test_data, test_labels) = data_preparation::train_test_split(vec![median_age], median_house_value, 0.8).unwrap();

    //println!("Shape train_data {:?}", train_data.shape());
    let regr = regression::Regression::train(train_data, train_labels);
    let prediction = regr.evaluate(test_data.clone());

    println!("Predictions {:?}", prediction[0]); //, test_data.row(0));

}   
