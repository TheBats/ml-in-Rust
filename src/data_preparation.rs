use nalgebra::{Dynamic, OMatrix};

pub type Matrixf64 = OMatrix<f64, Dynamic, Dynamic>;

pub fn train_test_split(data: Vec<Vec<f64>>, labels: Vec<f64>, ratio:f64) -> Result<(Matrixf64, Matrixf64, Matrixf64, Matrixf64), String> {
	
	if ratio > 1.0 {
		return Err("Please enter a ratio < 1".to_string())
	}

	let len_training: usize = (data[0].len() as f64*ratio) as usize;

	let mut train_data: Vec<f64> = Vec::new();
	let mut test_data: Vec<f64> = Vec::new();

	data.iter().for_each(|e| 
		e.iter().enumerate()
			.for_each(|e| {
				if e.0 <= len_training {
					train_data.push(e.1.to_owned())
				} else {
					test_data.push(e.1.to_owned())
				}
			})
		);

	let mut train_labels: Vec<f64> = Vec::new();
	let mut test_labels: Vec<f64> = Vec::new();

	labels.iter()
		.enumerate()
		.for_each(|e| {
		if e.0 <= len_training {
			train_labels.push(e.1.to_owned())
		} else {
			test_labels.push(e.1.to_owned())
		}
	});

	println!("\nTrain data {:?}", train_data.len());
	println!("Test data {:?}", test_data.len());
	println!("Train labels {:?}", train_labels.len());
	println!("Test data {:?}\n", test_labels.len());

	let train_data = Matrixf64::from_vec(train_data.len(), data.len(), train_data);
	let train_labels = Matrixf64::from_vec(train_labels.len(), 1, train_labels);
	let test_data = Matrixf64::from_vec(test_data.len(), data.len(), test_data);
	let test_labels = Matrixf64::from_vec(test_labels.len(), 1, test_labels);

	Ok((train_data, train_labels, test_data, test_labels))
}