/// Contains all the necessary information to sample a given point
pub struct Spline{
	/// Contains the positions of the data.
	/// This is used to figure out which coefficients need to be used at a given position
	x: Vec<f64>,

	/// The spline is made of cubic polynomial segments with coefficients
	/// A cubic Polynomial has 4 parameters and thus this is a vec containing 4 parameters for each segment
	coefficients: Vec<(f64, f64, f64, f64)>
}

impl Spline{
	/// Returns a Spline struct from a dataset
	///
	/// # Arguments
	///
	/// * `points` - A vec that holds each point `(f64, f64)` to be interpolated<br>
	/// The first element in the tuple being the key that will be used for finding the right segment during sampling
	///
	/// # Example
	///
	/// ```
	/// let spline = makima_spline::Spline::from_vec(vec![(1., 3.), (2., 5.), (3., 2.)]);
	///  ``` 
	///
	/// # Panics
	///
	/// [`panic!`] when there is only one point in the vector. Make sure your vector contains at least 2 points to interpolate between
	///
	/// [`panic!`]: https://doc.rust-lang.org/std/macro.panic.html
	pub fn from_vec(points: Vec<(f64, f64)>)->Spline{
		assert!(points.len() > 1, "Only one point. No Interpolation possible");
		let mut points = points;
		points.sort_by(|(x0, _), (x1, _)| x0.partial_cmp(x1).unwrap());
		// tangents
		let mut m = Vec::<f64>::with_capacity(points.len()+3);
		for i in 0..points.len()-1{
			m.push((points[i+1].1 - points[i].1)/(points[i+1].0-points[i].0));
		}
		if m.len() == 1{
			// this is just a line!
			let p1 = points[0];
			let p2 = points[1];
			let coefficients =  vec![(p1.1, m[0], 0., 0.)];
			return Spline{x: vec![p1.0, p2.0], coefficients}
		}
		// extrapolation at beginning
		m.insert(0, 2.0*m[0]-m[1]);
		m.insert(0, 2.0*m[0]-m[1]);
		// extrapolation at end
		m.push(2.0*m[m.len()-1]-m[m.len()-2]);
		m.push(2.0*m[m.len()-1]-m[m.len()-2]);
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

	///finds the segment and its parameters for interpolation
	///
	///the output is (a, b, c, d, xi) with xi the coordinate of the polynomial for dx
	fn segment(&self, pos: f64) -> (f64, f64, f64, f64, f64){
		if pos <= self.x[0]{
			// extrapolate linear
			let (a, b, _, _) = self.coefficients[0];
			let xi = self.x[0];
			return (a, b, 0.0, 0.0, xi);
		}
		if pos >= self.x[self.x.len()-1]{
			// extrapolate linear
			let (a, b, c, d) = self.coefficients[self.coefficients.len()-1];
			let xi = self.x[self.x.len()-2];
			let xi1 = self.x[self.x.len()-1];
			let dx = xi1-xi; 
			let y1 = a + b*dx + c*dx*dx + d*dx*dx*dx;
			let m = b + 2.*c*dx + 3.*d*dx*dx;
			return (y1, m, 0.0, 0.0, xi1);
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
		let coef = self.coefficients[lower];
		(coef.0, coef.1, coef.2, coef.3, self.x[lower])
	}

	///samples a point at a given position
	///
	/// # Example
	/// 
	/// ```
	/// let spline = makima_spline::Spline::from_vec(vec![(1., 3.), (2., 5.), (3., 2.)]);
	/// let sample: f64 = spline.sample(1.5);
	/// //This even works outside the given data (extrapolation)
	/// let sample2: f64 = spline.sample(-3.0);
	/// ```
	pub fn sample(&self, pos: f64)-> f64{
		let (a, b, c, d, xi) = self.segment(pos);
		
		let dx = pos-xi;
		a + b*dx + c*dx*dx + d*dx*dx*dx
	}

	/// returns the 1. order derivative at sampled point
	pub fn derivative_1(&self, pos: f64) -> f64{
		let (_, b, c, d, xi) = self.segment(pos);
		let dx = pos-xi;
		b + 2.*c*dx + 3.*d*dx*dx
	}

	/// returns the 2. order derivative at sampled point 
	///
	/// note, this doesn't have to be continous
	pub fn derivative_2(&self, pos: f64) -> f64{
		let (_, _, c, d, xi) = self.segment(pos);
		let dx = pos-xi;
		2.*c + 6.*d*dx
	}

	/// returns the 3. order derivative at sampled point 
	///
	/// note, this doesn't have to be continous
	pub fn derivative_3(&self, pos: f64) -> f64{
		let (_, _, _, d, _) = self.segment(pos);
		6.0*d
	}
}

/// converts two vecs of x & y coordinates into points (f64, f64) used to build the spline
///
/// # Example
/// ```
/// let x = vec![1., 2., 3.];
/// let y = vec![3., 5., 2.];
/// let points = makima_spline::vec_to_points(&x, &y);
/// ```
pub fn vec_to_points(x: &Vec<f64>, y: &Vec<f64>) -> Vec<(f64, f64)>{
	assert!(x.len() == y.len());
	let mut points = Vec::<(f64, f64)>::with_capacity(x.len());
	for i in 0..x.len(){
		points.push((x[i], y[i]))
	}
	points
}

mod tests;
