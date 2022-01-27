use crate::data_preparation::Matrixf64;

pub struct Regression {
	weights: Matrixf64
}

impl Regression {

	pub fn train(data: Matrixf64, labels: Matrixf64) -> Regression {
		
		//to do: Verify size !!

		let data = data.clone();
		let x = data.insert_column(0, 1.0);

		let x_tran = x.transpose();
		let inv = (x_tran.clone()*x).try_inverse().unwrap();
		let weights = inv*x_tran*labels;

		println!("Coefs: {:?}", weights.data);

		Regression {
			weights
		}
	}

	pub fn evaluate(&self, test: Matrixf64) -> Matrixf64 {

		let test = test.clone();
		let x = test.insert_column(0, 1.0);

		self.weights.clone().transpose()*x.transpose()
	}
} 