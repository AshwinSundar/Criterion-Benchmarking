use criterion_benchmarking::{euler1_par, euler1_series, euler1_simple};

fn main() {
    let input = 10003;

    println!("{}", euler1_simple(input));
    println!("{}", euler1_par(input));
    println!("{}", euler1_series(input));
}

// fn test_func() {
//     let x = (9999 / 3 * 3) * (3 + 9999 / 3 * 3) / 2;
//     println!("{}", x);
//     let x = (10000 / 3 * 3) * (3 + 10000 / 3 * 3) / 2;
//     println!("{}", x);
//     let x = (10002 / 3 * 3) * (3 + 10002 / 3 * 3) / 2;
//     println!("{}", x);
// }

// fn test_func2() {
// let x = f32::floor(9997. / 3.);
// println!("{}", x); // 3332
// let x = f32::floor(9998. / 3.);
// println!("{}", x); // 3332
// let x = f32::floor(9999. / 3.);
// println!("{}", x); // 3333
// let x = f32::floor(10000. / 3.);
// println!("{}", x); // 3333
// let x = f32::floor(10001. / 3.);
// println!("{}", x); // 3333
// let x = f32::floor(10002. / 3.);
// println!("{}", x); // 3334

// let x = (3. + 3. * f32::floor(9997. / 3.)) / 2.;
// println!("{}", x); // 4999.5
// let x = (3. + 3. * f32::floor(9998. / 3.)) / 2.;
// println!("{}", x); // 4999.5
// let x = (3. + 3. * f32::floor(9999. / 3.)) / 2.;
// println!("{}", x); // 5001
// let x = (3. + 3. * f32::floor(10000. / 3.)) / 2.;
// println!("{}", x); // 5001
// let x = (3. + 3. * f32::floor(10001. / 3.)) / 2.;
// println!("{}", x); // 5001
// let x = (3. + 3. * f32::floor(10002. / 3.)) / 2.;
// println!("{}", x); // 5002.5

// let x = (f32::floor(9997. / 3.)) * (3. + 3. * f32::floor(9997. / 3.));
// println!("{}", x); // 33316668
// let x = (f32::floor(9998. / 3.)) * (3. + 3. * f32::floor(9998. / 3.));
// println!("{}", x); // 33316668
// let x = (f32::floor(9999. / 3.)) * (3. + 3. * f32::floor(9999. / 3.));
// println!("{}", x); // 33316666
// let x = (f32::floor(10000. / 3.)) * (3. + 3. * f32::floor(10000. / 3.));
// println!("{}", x); // 33316666
// let x = (f32::floor(10001. / 3.)) * (3. + 3. * f32::floor(10001. / 3.));
// println!("{}", x); // 33316666
// let x = (f32::floor(10002. / 3.)) * (3. + 3. * f32::floor(10002. / 3.));
// println!("{}", x); // 33356670

// let x = (f32::floor(9997. / 3.)) * (3. + 3. * f32::floor(9997. / 3.)) / 2.;
// println!("{}", x); // 16658334
// let x = (f32::floor(9998. / 3.)) * (3. + 3. * f32::floor(9998. / 3.)) / 2.;
// println!("{}", x); // 16658334
// let x = (f32::floor(9999. / 3.)) * (3. + 3. * f32::floor(9999. / 3.)) / 2.;
// println!("{}", x); // 16668333
// let x = (f32::floor(10000. / 3.)) * (3. + 3. * f32::floor(10000. / 3.)) / 2.;
// println!("{}", x); // 16668333
// let x = (f32::floor(10001. / 3.)) * (3. + 3. * f32::floor(10001. / 3.)) / 2.;
// println!("{}", x); // 16668333
// let x = (f32::floor(10002. / 3.)) * (3. + 3. * f32::floor(10002. / 3.)) / 2.;
// println!("{}", x); // 16678335
// }
