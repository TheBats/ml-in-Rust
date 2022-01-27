use std::{error::Error, str::FromStr};

use csv::StringRecord;
use rayon::prelude::*;

pub struct DataFrame {
	rows: Vec<StringRecord>,
	pub headers: Vec<String>
}

impl DataFrame {

	pub fn from_path(path: &str) -> Result<DataFrame, Box<dyn Error>> {
		
		let mut rdr = csv::Reader::from_path(path)?;
		
		let headers: Vec<String> = rdr.headers().unwrap()
			.clone()
			.iter()
			.map(|e| e.to_string())
			.collect();

		let rows: Vec<StringRecord> = rdr.records()
			.into_iter()
			.map(|row|  {
				row.unwrap()
			}).collect();

		Ok(DataFrame {
			rows,
			headers
		})
	}

	pub fn to<T>(&self, col_name: &str) -> Vec<T>
		where 
			T: FromStr + std::marker::Send
	{

		let index = self.headers.iter().position(|v|v == col_name).unwrap();

		let res: Vec<T> = self.rows.par_iter()
			.map(|row| {
				let entry = row[index].to_string();
				match entry.parse::<T>() {
					Ok(fl) => fl,
					Err(_) => panic!("{}", format!("Unable to parse {entry} to float").to_string())
				}
			}).collect();

		res
	}
}