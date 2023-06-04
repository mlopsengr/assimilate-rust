use easy_ml:matrices::Matrix;

let x: Matrix<f32> = Matrix::Column(
    vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0]
)

let y = x.map(|x| x.powi(2) + x.sin());
println!("{:?}", &y);

let mut X = x.clone();

X.insert_column(0, 1.0);

X.insert_column_with(2, x.column_iter(0).map(|x| x * x))
println!("{:?}", &X);


let w = (X.transpose() * &X).inverse().unwrap() * (X.transpose() * &y);

let predictions = &X * &w;

let errors = &y - &predictions;

let mean_squared_error = (errors.transpose() * &errors).get(0, 0) / x.rows() as f32;

println!("MSE: {}", mean_squared_error);
assert!(mean_squared_error > 0.41);


fn main() {
    println!("Hello, world!");
}
