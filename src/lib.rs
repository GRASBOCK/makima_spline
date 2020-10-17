pub struct Spline{
	x: Vec<f64>,
	coefficients: Vec<(f64, f64, f64, f64)>
}

impl Spline{
	pub fn from_vec(points: Vec<(f64, f64)>)->Spline{
		let mut points = points;
		points.sort_by(|(x0, _), (x1, _)| x0.partial_cmp(x1).unwrap());
		// tangents
		let mut m = Vec::<f64>::with_capacity(points.len()+3);
		for i in 0..points.len()-1{
			m.push((points[i+1].1 - points[i].1)/(points[i+1].0-points[i].0));
		}
		// extrapolation at beginning
		m.insert(0, 2.0*m[0]-m[1]);
		m.insert(0, 2.0*m[0]-m[1]);
		// extrapolation at end
		println!("m:{:?}",m);
		m.push(2.0*m[m.len()-1]-m[m.len()-2]);
		println!("m:{:?}",m);
		m.push(2.0*m[m.len()-1]-m[m.len()-2]);
		println!("m:{:?}",m);
		let m = m; // make immutable

		// derivatives at points 
		let mut t= Vec::<f64>::with_capacity(points.len()-1);
		for i in 0..points.len(){
			let i = i + 2; // because of extrapolation
			let w1 = (m[i+1]-m[i]).abs() + (m[i+1]+m[i]).abs()/2.;
			let w2 = (m[i-1]-m[i-2]).abs() + (m[i-1]+m[i-2]).abs()/2.;
			let nominator = w1*m[i-1] + w2*m[i];
			if nominator == 0.0{
				t.push(0.0)
			}else{
				t.push(nominator/(w1+w2))
			}
			
		}
		let t = t; // make immutable
		println!("t: {:?}", t);

		// coefficients
		let mut coefficients = Vec::<(f64, f64, f64, f64)>::with_capacity(points.len()-1);
		for i in 0..points.len()-1{
			let (xi, yi) = points[i];
			let (xi1, _) = points[i+1];
			let z = xi1-xi;
			let a = yi;
			let b = t[i];
			let c = (3.*m[i+2] - 2.*t[i]-t[i+1])/z;
			let d = (t[i]+t[i+1] - 2.*m[i+2])/z.powi(2);
			coefficients.push((a, b, c, d));
		}

		// extract x
		let x = points.iter().map(|p|{
			p.0
		}).collect();
		Spline{x, coefficients}
	}

	pub fn sample(&self, pos: f64)-> f64{
		let y = |xi: f64, x: f64, (a, b, c, d): (f64, f64, f64, f64)| -> f64{
			let dx = x-xi;
			a + b*dx + c*dx*dx + d*dx*dx*dx
		};
		if pos <= self.x[0]{
			// extrapolate linear
			let (a, b, c, d) = self.coefficients[0];
			let xi = self.x[0];
			let dx = -xi; 
			let m = b + 2.*c*dx + 3.*d*dx*dx;
			return a + (pos-xi)*m;
		}
		if pos >= self.x[self.x.len()-1]{
			// extrapolate linear
			let (a, b, c, d) = self.coefficients[self.coefficients.len()-1];
			let xi = self.x[self.x.len()-2];
			let xi1 = self.x[self.x.len()-1];
			let dx = xi1-xi; 
			let m = b + 2.*c*dx + 3.*d*dx*dx;
			return y(xi, xi1, (a, b, c, d)) + (pos-xi1)*m;
		}
		//do binary search
		let mut upper = self.x.len();
		let mut lower = 0;
		while upper-lower > 1{
			let center = (upper+lower)/2;
			if pos > self.x[center]{
				lower = center;
			}else{
				upper = center;
			}
		}
		y(self.x[lower], pos, self.coefficients[lower])
	}
}

pub fn vec_to_points(x: &Vec<f64>, y: &Vec<f64>) -> Vec<(f64, f64)>{
	assert!(x.len() == y.len());
	let mut points = Vec::<(f64, f64)>::with_capacity(x.len());
	for i in 0..x.len(){
		points.push((x[i], y[i]))
	}
	points
}

#[cfg(test)]
mod tests {

	use crate::Spline;

	fn output(file: String, spline: &Spline, x_max: f64, x_min: f64, delta: f64){
		let steps = ((x_max - x_min)/delta) as usize;
		let mut contents = String::default();
		for i in 0..steps{
			let x = i as f64*delta+x_min;
			let y = spline.sample(x);
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

		output(format!("step"), &spline, x[x.len()-1], x[0], 0.01)
	}

	#[test]
	fn general() {
		let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 5.5, 7.0, 8.0, 9.0, 9.5, 10.0];
		let y = vec![-0., 0., 0., 0.5, 0.4, 1.2, 1.2, 0.1, 0., 0.3, 0.6];

		let points = crate::vec_to_points(&x, &y);

		let spline = Spline::from_vec(points);
		
		output(format!("general"), &spline, x[x.len()-1]+1.5, x[0]-1.0, 0.01)
	}
}