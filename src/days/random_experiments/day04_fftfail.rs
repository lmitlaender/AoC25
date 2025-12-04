use std::{f64::consts::PI};

use super::Day;
use num::complex::Complex;

pub struct Day04;

impl Day04 {
    /// Pad a 2D array to the given number of rows and columns
    pub fn pad_to_size<T: Copy + Default>(array: &Vec<Vec<T>>, rows: usize, cols: usize) -> Vec<Vec<T>> {
        let mut padded = vec![vec![T::default(); cols]; rows];
        let orig_rows = array.len();
        let orig_cols = if orig_rows > 0 { array[0].len() } else { 0 };
        for i in 0..orig_rows.min(rows) {
            for j in 0..orig_cols.min(cols) {
                padded[i][j] = array[i][j];
            }
        }
        padded
    }

    /// Unpad a 2D array to the given number of rows and columns
    pub fn unpad_to_size<T: Copy>(array: &Vec<Vec<T>>, rows: usize, cols: usize) -> Vec<Vec<T>> {
        array.iter()
            .take(rows)
            .map(|row| row.iter().take(cols).copied().collect())
            .collect()
    }
    fn parse(input: &str) -> Vec<Vec<u8>> {
        let grid = input.lines().map(|line| {
            line.chars().map(|c| {
                match c {
                    '@' => 1u8,
                    _ => 0u8
                }
            }).collect::<Vec<u8>>()
        }).collect::<Vec<Vec<u8>>>();

        // prepend and postpend with 0 vector
        grid
    }

    fn fft_base(array: &Vec<Complex<f64>>, inverse: bool) -> Vec<Complex<f64>> {
        let n = array.len();
        if n == 1 {
            array.clone()
        } else {
            let mut res: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); n];
            let even_array: Vec<Complex<f64>> = array.iter().step_by(2).copied().collect();
            let odd_array: Vec<Complex<f64>> = array.iter().skip(1).step_by(2).copied().collect();
            let even_res = Self::fft_base(&even_array, inverse);
            let odd_res = Self::fft_base(&odd_array, inverse);
                        
            // Combine results
            for k in 0..(n/2) {
                let p = even_res[k];
                let twiddle = match inverse {
                    true => Complex::from_polar(1.0, 2.0 * PI * k as f64 / n as f64),
                    false => Complex::from_polar(1.0, -2.0 * PI * k as f64 / n as f64)
                };
                let q = twiddle * odd_res[k];
                res[k] = p + q;
                res[k + n / 2] = p - q;
            }
            res
        }
    }

    fn fft(array: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
        Self::fft_base(array, false)
    }
    
    fn ifft(array: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
        Self::fft_base(array, true)
    }

    fn transpose(matrix: &Vec<Vec<Complex<f64>>>) -> Vec<Vec<Complex<f64>>> {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut transposed = vec![vec![Complex::new(0.0, 0.0); rows]; cols];
        for i in 0..rows {
            for j in 0..cols {
                transposed[j][i] = matrix[i][j];
            }
        }
        transposed
    }

    fn fft2(array: &Vec<Vec<Complex<f64>>>) -> Vec<Vec<Complex<f64>>> {
        let row_fft: Vec<Vec<Complex<f64>>> = array.iter().map(|row| Self::fft(row)).collect();
        let transposed = Self::transpose(&row_fft);
        let col_fft: Vec<Vec<Complex<f64>>> = transposed.iter().map(|row| Self::fft(row)).collect();
        Self::transpose(&col_fft)
    }

    fn ifft2(array: &Vec<Vec<Complex<f64>>>) -> Vec<Vec<Complex<f64>>> {
        let row_ifft: Vec<Vec<Complex<f64>>> = array.iter().map(|row| Self::ifft(row)).collect();
        let transposed = Self::transpose(&row_ifft);
        let mut col_ifft: Vec<Vec<Complex<f64>>> = transposed.iter().map(|row| Self::ifft(row)).collect();
        let rows = col_ifft.len();
        let cols = if rows > 0 { col_ifft[0].len() } else { 0 };
        let scale = (rows * cols) as f64;
        for i in 0..rows {
            for j in 0..cols {
                col_ifft[i][j].re /= scale;
                col_ifft[i][j].im /= scale;
            }
        }
        Self::transpose(&col_ifft)
    }

    
}

impl Day for Day04 {
    fn part1(&self, input: &str) -> String {
        // Parse the input data using the parse function
        let data = Self::parse(input);

        // Ensure the kernel is defined (example kernel, adjust as needed)
        let kernel = vec![vec![1.0, 1.0, 1.0], vec![1.0, 0.0, 1.0], vec![1.0, 1.0, 1.0]];

        // Calculate the maximum dimensions for padding
        let max_rows = data.len().max(kernel.len());
        let max_cols = data[0].len().max(kernel[0].len());
        let pad_rows = max_rows.next_power_of_two();
        let pad_cols = max_cols.next_power_of_two();

        // Pad both data and kernel to the same dimensions
        let padded_data = Self::pad_to_size(&data, pad_rows, pad_cols);
        let padded_kernel = Self::pad_to_size(&kernel, pad_rows, pad_cols);

        // Convert padded data and kernel to real numbers for FFT
        let data_complex: Vec<Vec<num::Complex<f64>>> = padded_data.iter()
            .map(|row| row.iter().map(|&x| num::Complex::new(x as f64, 0.0)).collect())
            .collect();
        let kernel_complex: Vec<Vec<num::Complex<f64>>> = padded_kernel.iter()
            .map(|row| row.iter().map(|&x| num::Complex::new(x as f64, 0.0)).collect())
            .collect();

        // Perform 2D FFT on the data and kernel
        let data_fft = Self::fft2(&data_complex);
        let kernel_fft = Self::fft2(&kernel_complex);

        // Perform element-wise multiplication in the frequency domain
        let mut result_fft = vec![vec![num::Complex::new(0.0, 0.0); pad_cols]; pad_rows];
        for i in 0..pad_rows {
            for j in 0..pad_cols {
                result_fft[i][j] = data_fft[i][j] * kernel_fft[i][j];
            }
        }

        // Perform inverse FFT to get the convolution result
        let result_padded = Self::ifft2(&result_fft);

        // Unpad the result to the original output size
        let out_rows = data.len() + kernel.len() - 1;
        let out_cols = data[0].len() + kernel[0].len() - 1;
        let result = Self::unpad_to_size(&result_padded, out_rows, out_cols);

        // Count cells where the real part is smaller or equal to 4
        let count = result.iter()
            .flat_map(|row| row.iter().map(|val| val.re))
            .filter(|&real| real <= 4.0)
            .count();

        // Print the convolution result
        println!("Convolution Result (Real Part):");
        for row in &result {
            let formatted_row: Vec<String> = row.iter().map(|val| format!("{:.2}", val.re)).collect();
            println!("{:?}", formatted_row);
        }

        println!("Convolution Result (Imaginary Part):");
        for row in &result {
            let formatted_row: Vec<String> = row.iter().map(|val| format!("{:.2}", val.im)).collect();
            println!("{:?}", formatted_row);
        }

        // Print the input data
        println!("Input Data:");
        for row in &data {
            println!("{:?}", row);
        }

        // Overlay the real result with the original mask
        let masked_result: Vec<Vec<f64>> = result.iter()
            .zip(data.iter())
            .map(|(result_row, data_row)| {
                result_row.iter()
                    .zip(data_row.iter())
                    .map(|(result_cell, &data_cell)| if data_cell == 0 { 0.0 } else { result_cell.re })
                    .collect()
            })
            .collect();

        // Count cells where the real part is smaller or equal to 4 in the masked result
        let count = masked_result.iter()
            .flat_map(|row| row.iter())
            .filter(|&&real| real <= 4.0)
            .count();

        // Print the masked convolution result
        println!("Masked Convolution Result:");
        for row in &masked_result {
            let formatted_row: Vec<String> = row.iter().map(|&val| format!("{:.2}", val)).collect();
            println!("{:?}", formatted_row);
        }

        // Return the count as a string
        count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        "42".into()
    }
}

#[cfg(test)]
mod tests {
        // Direct 2D convolution for comparison
        fn direct_convolution(image: &Vec<Vec<f64>>, kernel: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
            let image_rows = image.len();
            let image_cols = image[0].len();
            let kernel_rows = kernel.len();
            let kernel_cols = kernel[0].len();
            let out_rows = image_rows + kernel_rows - 1;
            let out_cols = image_cols + kernel_cols - 1;
            let mut output = vec![vec![0.0; out_cols]; out_rows];
            for i in 0..out_rows {
                for j in 0..out_cols {
                    let mut sum = 0.0;
                    for ki in 0..kernel_rows {
                        for kj in 0..kernel_cols {
                            let ii = i as isize - ki as isize;
                            let jj = j as isize - kj as isize;
                            if ii >= 0 && ii < image_rows as isize && jj >= 0 && jj < image_cols as isize {
                                sum += image[ii as usize][jj as usize] * kernel[ki][kj];
                            }
                        }
                    }
                    output[i][j] = sum;
                }
            }
            output
        }
    use super::*;
    #[test]
    fn test_2d_convolution_fft() {
        use std::time::Instant;

        // Example 3x3 image
        let image = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];

        // Example 3x3 kernel (Sobel edge detection)
        let kernel = vec![
            vec![1.0, 0.0, -1.0],
            vec![2.0, 0.0, -2.0],
            vec![1.0, 0.0, -1.0],
        ];



        // Calculate output size and next power of two for padding
        let out_rows = image.len() + kernel.len() - 1;
        let out_cols = image[0].len() + kernel[0].len() - 1;
        let pad_rows = out_rows.next_power_of_two();
        let pad_cols = out_cols.next_power_of_two();

        // Flip the kernel for convolution
        let mut flipped_kernel = vec![vec![0.0; kernel[0].len()]; kernel.len()];
        for i in 0..kernel.len() {
            for j in 0..kernel[0].len() {
                flipped_kernel[i][j] = kernel[kernel.len() - 1 - i][kernel[0].len() - 1 - j];
            }
        }

        // Pad image and kernel to next power of two
        let padded_image = Day04::pad_to_size(&image, pad_rows, pad_cols);
        let padded_kernel = Day04::pad_to_size(&kernel, pad_rows, pad_cols);

        // Convert to Complex<f64>
        let image_c: Vec<Vec<Complex<f64>>> = padded_image.iter()
            .map(|row| row.iter().map(|&x| Complex::new(x, 0.0)).collect())
            .collect();
        let kernel_c: Vec<Vec<Complex<f64>>> = padded_kernel.iter()
            .map(|row| row.iter().map(|&x| Complex::new(x, 0.0)).collect())
            .collect();

        // Time FFT-based convolution
        let start_fft = Instant::now();

        // FFT2 both
        let image_fft = Day04::fft2(&image_c);
        let kernel_fft = Day04::fft2(&kernel_c);

        // Pointwise multiply
        let mut result_fft = vec![vec![Complex::new(0.0, 0.0); pad_cols]; pad_rows];
        for i in 0..pad_rows {
            for j in 0..pad_cols {
                result_fft[i][j] = image_fft[i][j] * kernel_fft[i][j];
            }
        }

        // Inverse FFT2 (now automatically scaled)
        let result_padded = Day04::ifft2(&result_fft);
        // Unpad to output size
        let result = Day04::unpad_to_size(&result_padded, out_rows, out_cols);

        let duration_fft = start_fft.elapsed();

        // Time direct convolution
        let start_direct = Instant::now();
        let expected = direct_convolution(&image, &kernel);
        let duration_direct = start_direct.elapsed();

        // Compare results
        for i in 0..result.len() {
            for j in 0..result[0].len() {
                assert!((result[i][j].re - expected[i][j]).abs() < 1e-6, "Mismatch at ({}, {}): FFT = {}, Direct = {}", i, j, result[i][j].re, expected[i][j]);
            }
        }

        println!("FFT-based convolution time: {:?}", duration_fft);
        println!("Direct convolution time: {:?}", duration_direct);
    }
}