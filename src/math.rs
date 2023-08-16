use rand::Rng;
use std::fmt;

pub trait MathOps<RHS, Output> {
    fn add(&self, rhs: RHS) -> Output;
    fn sub(&self, rhs: RHS) -> Output;
    fn mul(&self, rhs: RHS) -> Output;
    fn dot_product(&self, rhs: RHS) -> Output;
}

pub struct Matrix {
    pub data: Vec<Vec<f64>>,
}

impl MathOps<&Matrix, Matrix> for Matrix {
    fn sub(&self, rhs: &Matrix) -> Matrix {
        assert_eq!(self.data.len(), rhs.data.len());
        assert_eq!(self.data[0].len(), rhs.data[0].len());

        let mut result = Matrix::new(self.data.len(), self.data[0].len());

        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                result.data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }

        result
    }

    fn add(&self, rhs: &Matrix) -> Matrix {
        assert_eq!(self.data.len(), rhs.data.len());
        assert_eq!(self.data[0].len(), rhs.data[0].len());

        let mut result = Matrix::new(self.data.len(), self.data[0].len());

        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                result.data[i][j] = self.data[i][j] * rhs.data[i][j];
            }
        }

        result
    }

    fn mul(&self, rhs: &Matrix) -> Matrix {
        assert_eq!(self.data.len(), rhs.data.len());
        assert_eq!(self.data[0].len(), rhs.data[0].len());

        let mut result = Matrix::new(self.data.len(), self.data[0].len());

        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                let mut sum = 0.0;
                for k in 0..self.data[0].len() {
                    sum += self.data[i][k] * rhs.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }

        result
    }

    fn dot_product(&self, rhs: &Matrix) -> Matrix {
        assert_eq!(self.data.len(), rhs.data[0].len());

        let mut result = Matrix::new(self.data.len(), rhs.data[0].len());

        for row in 0..result.data.len() {
            for col in 0..result.data[row].len() {
                let mut sum = 0.0;
                for k in 0..self.data[0].len() {
                    sum += self.data[row][k] * rhs.data[k][col];
                }
                result.data[row][col] = sum;
            }
        }
        result
    }
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn from_vec(input:Vec<f32>) -> Matrix {
        Matrix::new(input.len(),0)
    }

    pub fn randomize(&self) -> Matrix {
        let mut rng = rand::thread_rng();
        let mut result = Matrix::new(self.data.len(), self.data[0].len());
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                result.data[i][j] = rng.gen_range(0.0..1.0);
            }
        }
        result
    }


    fn identity(size: usize) -> Matrix {
        let mut matrix = Matrix::new(size, size);
        for i in 0..size {
            matrix.data[i][i] = 1.0;
        }
        matrix
    }

    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.data[0].len(), self.data.len());
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.data {
            for &element in row {
                write!(f, "{:.2} ", element)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

