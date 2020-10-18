# makima_spline
A implementation of the modified akima interpolation<br>
**+** additionally features linear extrapolation

<img src="testing/general.png">


## HowTo
```rust
use makima_spline::Spline;
```
your data is in some format
```rust
let x = vec![1., 2., 3., 4., 5., 6., 7., 8.];
let y = vec![-1., -1., -1., 0.0, 1., 1., 1., 1.];
``` 
convert to the type used by the spline `Vec<(f64, f64)>`
```rust
let points = makima_spline::vec_to_points(&x, &y);
```    
build the spline from the data-points
```rust
let spline = Spline::from_vec(points);
```
To sample do this:
```rust
let y = spline.sample(x);
```
<img src="testing/step.png">
