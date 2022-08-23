extern crate crossbeam;
extern crate image;
extern crate num;

use std::fs::File;
use std::io::Write;
use std::str::FromStr;

use image::ColorType;
use image::png::PNGEncoder;
use num::Complex;

/**
复数：实数+虚数
实数：有理数+无理数
有理数：整数+有限小数+无限循环小数
无理数：无限不循环小数
整数：0+正数+负数
虚数： 对负数开平方根得到的数是虚数，也就是说反过来虚数的平方还是负数。
* 第一个多线程程度

*/
///
///
/// 曼德布洛特集合 ：有这样一个复数集合，里面的任意一个数 z ，经过无限次曼德布洛特算法
/// 之后的值c，不会接近无穷大。
/// 在main函数中使用return是初学者常犯的错误，因为Main函数没有返回值
///
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(std::io::stderr(), "Usage:曼德洛布特集合 绘制参数不够")
            .unwrap();
        writeln!(std::io::stderr(), "eg: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }
    let bounds: (usize, usize) = parse_pair(&args[2], 'x')
        .expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3])
        .expect("error parsing upper_left ");
    let lower_right = parse_complex(&args[4])
        .expect("error parsing lower_right");
    let mut pixels = vec![0; bounds.0 * bounds.1];
    // vec![v,n]  宏调用，会创建一个长度为n，每个元素都是 v的 向量

    renderAsync(&mut pixels, bounds, upper_left, lower_right);
    write_image(&args[1], &pixels, bounds)
        .expect("error writing png file");
}

///
/// Optional rust标准库中的 可枚举类型
///
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        // z 的值平方大于了半径为2的圆，那么他一定最终分向无穷大，所以不是 曼度布洛特集合，跳过，返回的i 就是循环次数
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

///
/// 曼德布洛特算法
#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    //惯例是 z  表示复数
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

///
///  Option<(T, T)>  是包含两个T 类型的元组
///
///
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None   //通配符模式匹配任何值，但忽略值
            }
        }
    }
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("0.123,-1.89"), Some(Complex { re: 0.123, im: -1.89 }));
    assert_eq!(parse_complex(",9.87"), None);
}

#[test]
fn test_parse_pair() {
    let option = parse_pair::<i32>("", ',');
    assert_eq!(option, None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x9.5", 'x'), Some((0.5, 9.5)));
}

fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize),
                  upper_left: Complex<f64>, lower_right: Complex<f64>) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
        //这里为什么要用减法？ pixel.1越往下越大，而虚部越往上越大
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 100), (25, 75),
                              Complex { re: -1.0, im: 1.0 },
                              Complex { re: 1.0, im: -1.0 }),
               Complex { re: -0.5, im: -0.5 });
}

///
///
/// 单线程版本
///
fn renderSync(pixel: &mut [u8], bounds: (usize, usize),
              upper_left: Complex<f64>, lower_right: Complex<f64>) {
    assert_eq!(pixel.len(), bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (row, column), upper_left, lower_right);
            // 如果是曼德布洛特集合，就渲染为黑色（0），否则根据遍历次数，决定颜色深度，最低255-255=0次，意味着白色
            pixel[row * bounds.0 + column] =
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
                };
        }
    }
}

///
/// |spawner| {...} 是 一个Rust闭包，闭包是一个可以被当做函数调用的值。|spawner| 是参数列表{...}是函数体
/// 跟使用 fn声明的函数不同，不需要声明闭包的参数类型，Rust会推断闭包的参数以及返回值类型
///
fn renderAsync(pixel: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) {
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;
    {
        let bands: Vec<&mut [u8]> = pixel.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
                spawner.spawn(move || {
                    renderSync(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        });
    }
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    // 上面的代码，以前的写法有, 简写   File::create(filename)?，  注意后面这个  ?
    // let output = match File::create(filename)
    // {
    //     Ok(f) => { f }
    //     Err(e) => { return Err(e); }
    // };

    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
    Ok(())
}

// fn write_image2(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<> {
//     let output = File::create(filename)?;
//     // 上面的代码，以前的写法有, 简写   File::create(filename)?，  注意后面这个  ?
//     // let output = match File::create(filename)
//     // {
//     //     Ok(f) => { f }
//     //     Err(e) => { return Err(e); }
//     // };
//
//     let encoder = PNGEncoder::new(output);
//     encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
//     Ok(())
// }
