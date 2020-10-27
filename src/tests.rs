#[cfg(test)]
mod tests{
	use crate::Spline;

	fn output(file: String, sampling: &dyn Fn(f64)-> f64, x_min: f64, x_max: f64, delta: f64){
		let steps = ((x_max - x_min)/delta) as usize;
		let mut contents = String::default();
		for i in 0..steps{
			let x = i as f64*delta+x_min;
			let y = sampling(x);
			contents.push_str(format!("{} {}\n", x, y).as_str());
		}
		std::fs::write(&file, contents).unwrap();
	}

	#[test]
	fn step(){
		let x = vec![1., 2., 3., 4., 5., 6., 7., 8.];
		let y = vec![-1., -1., -1., 0.0, 1., 1., 1., 1.];

		let points = crate::vec_to_points(&x, &y);

		let spline = Spline::from_vec(points);

		
		output(format!("testing/step"), &|x|spline.sample(x), x[0], x[x.len()-1], 0.01)
	}

	#[test]
	fn general() {
		let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 5.5, 7.0, 8.0, 9.0, 9.5, 10.0];
		let y = vec![-0., 0., 0., 0.5, 0.4, 1.2, 1.2, 0.1, 0., 0.3, 0.6];

		let points = crate::vec_to_points(&x, &y);

		let spline = Spline::from_vec(points);
		
		output(format!("testing/general"), &|x|spline.sample(x), x[0]-1.0, x[x.len()-1]+1.5, 0.01);
		output(format!("testing/general_deriv1"), &|x|spline.derivative_1(x), x[0]-1.0, x[x.len()-1]+1.5, 0.01);
		output(format!("testing/general_deriv2"), &|x|spline.derivative_2(x), x[0]-1.0, x[x.len()-1]+1.5, 0.01);
		output(format!("testing/general_deriv3"), &|x|spline.derivative_3(x), x[0]-1.0, x[x.len()-1]+1.5, 0.01);
	}

	#[test]
	fn line(){
		let x = vec![1., 2.];
		let y = vec![-1., -2.];

		let points = crate::vec_to_points(&x, &y);

		let spline = Spline::from_vec(points);

		output(format!("testing/line"), &|x|spline.sample(x), x[0]-1.0, x[x.len()-1]+1.0, 0.01)
	}
	#[test]
	fn basic(){
		let spline = Spline::from_vec(vec![(1., 3.), (2., 5.), (3., 2.)]);

		output(format!("testing/basic"), &|x|spline.sample(x), 0.0, 4.0, 0.01)
	}

}