
use bicubic;

/// creates a bicubic interpolation from grid data with makima spline derivatives
/// 
/// ## Usage
/// ```
/// let x = vec![-1.0, 0.0, 2.0];
///	let y = vec![-0.0, 1.0];
///	let f = vec![5.0, 4.0, 5.0, 1.0, 1.0, 1.0];
/// let bci = makima_spline::n_dimensional::bicubic_from_grid(&x, &y, &f);
/// ``` 
pub fn bicubic_from_grid(x: &Vec<f64>, y: &Vec<f64>, f: &Vec<f64>) -> bicubic::Bicubic{
	assert!(f.len() == x.len()*y.len(), "[makima_spline::n_dimensional::bicubic_from_grid] f has bad dimensions (f.len() = {}). Needs to be x.len()*y.len() = {}", f.len(), x.len()*y.len());
	// create Splines
	let mut row_splines = Vec::with_capacity(y.len());
	for row in 0..y.len() {
		let first = row*x.len();
		let last = first + x.len();
		let row_f = f[first..last].to_vec();
		let points: Vec<(f64,f64)> = x.iter().enumerate().map(|it|{
			let (index, x_val) = it;
			let f_val = row_f[index];
			(*x_val, f_val)
		}).collect();
		let row_spline = crate::Spline::from_vec(points);
		row_splines.push(row_spline);
	}
	let mut column_splines =  Vec::with_capacity(x.len());
	for column in 0..x.len() {

		let mut column_f = Vec::new();
		for y_index in 0..y.len() {
			column_f.push(f[y_index*x.len() + column]);
		}
		let points: Vec<(f64,f64)> = y.iter().enumerate().map(|it|{
			let (index, y_val) = it;
			let f_val = column_f[index];
			(*y_val, f_val)
		}).collect();
		let column_spline = crate::Spline::from_vec(points);
		column_splines.push(column_spline);
	}
	// calculate partial and cross derivatives
	let mut fx = Vec::with_capacity(f.len());
	let mut fy = Vec::with_capacity(f.len());
	for row in 0..y.len(){
		for column in 0..x.len(){
			let x_val = x[column];
			let fx_val = row_splines[row].derivative_1(x_val);
			fx.push(fx_val);
			let y_val = y[row];
			let fy_val = column_splines[column].derivative_1(y_val);
			fy.push(fy_val);
		}
	}
	let fxy = vec![0.0; f.len()];
	bicubic::from_vec(&x, &y, &f, &fx, &fy, &fxy)
}


#[cfg(test)]
fn output(file: String, sampling: &dyn Fn(f64, f64)-> f64, x_min: f64, x_max: f64, y_min: f64, y_max: f64, delta: f64){
	let x_steps = ((x_max - x_min)/delta) as usize;
	let y_steps = ((y_max - y_min)/delta) as usize;
	println!("x_steps: {}, y_steps {}", x_steps, y_steps);
	let mut contents = String::default();
	for yi in 0..y_steps{
		for xi in 0..x_steps{
			let x = xi as f64*delta+x_min;
			let y = yi as f64*delta+y_min;
			let z= sampling(x, y);
			contents.push_str(format!("{} {} {}\n", x, y, z).as_str());
		}
	}
	std::fs::write(&file, contents).unwrap();
}

#[test]
fn bicub(){
	let x = vec![-4.0, -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0];
	let y = vec![-0.0, 1.0];
	let f = vec![
						0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0,
						0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0];

	let interp = bicubic_from_grid(&x, &y, &f);
	output(format!("testing/manual"), &|x, y|interp.sample(x, y), -3., 3.0, -5., 4., 0.1);
}