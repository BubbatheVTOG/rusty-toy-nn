mod math;

pub struct nn3 {
    input: i32,
    hidden: i32,
    output: i32,
    bias_h: Matrix,
    bias_o: Matrix,
    weights_ih: Matrix,
    weights_ho: Matrix,

}

impl nn3 {
    fn new(input:usize, hidden:usize, output:usize, bias:usize, weights:usize) -> nn3 {
        nn3 {
            weights_ih: Matrix::new(input,hidden).randomize(),
            weights_ho: Matrix::new(hidden,output).randomize(),
            bias_h: Matrix::new(hidden,1).random(),
            bias_o: Matrix::new(hidden,1).random(),
        }
    }

    pub fn feedforward(input) -> nn3 {
        let input = Matrix::from_vec(input);

        let hidden =

    }
}
