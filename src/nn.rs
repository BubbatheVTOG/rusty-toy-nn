use crate::math::{Matrix,MathOps};

pub struct NN3 {
    bias_h: Matrix,
    bias_o: Matrix,
    weights_ih: Matrix,
    weights_ho: Matrix,

}

impl NN3 {
    fn new(input:usize, hidden:usize, output:usize, bias:usize, weights:usize) -> NN3 {
        NN3 {
            weights_ih: Matrix::new(input,hidden).randomize(),
            weights_ho: Matrix::new(hidden,output).randomize(),
            bias_h: Matrix::new(hidden,1).randomize(),
            bias_o: Matrix::new(hidden,1).randomize(),
        }
    }

    pub fn feedforward(self, input:Vec<f32>) -> Matrix {
        let input = Matrix::from_vec(input);

        let hidden = self.weights_ih.mul(&input);
        let hidden = self.weights_ih.add(&self.bias_h);

        //hidden.map(sigmoid);

        let output = Matrix::from_vec(hidden);

        let output = self.weights_ho.mul(&hidden);
        let output = self.weights_ho.add(&self.bias_o);

        output
    }
}
