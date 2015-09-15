extern crate rand;

#[derive(Debug)]
pub struct Field<'a, T> {
    dim: &'a Dimension,
    field: Vec<T>
}

pub trait Access2D<T>
where T: Copy {

    fn get_at(&self, x: usize, y: usize) -> T;
    fn set_at(&mut self, x: usize, y: usize, value: T);
}

fn get_index(dim: &Dimension, x: usize, y: usize) -> usize {
    dim.width * y + x
}

impl <'a, T: Copy> Access2D<T> for Field<'a, T> {

    fn get_at(&self, x: usize, y: usize) -> T {
        self.field[ get_index( self.dim, x, y ) ]
    }

    fn set_at(&mut self, x: usize, y: usize, value: T) {
        self.field[ get_index( self.dim, x, y ) ] = value;
    }
}

impl <'a, T: Copy + Default> Field<'a, T> {

    pub fn new(dim: &'a Dimension) -> Field<'a, T> {
        Field {
            dim: dim,
            field: vec![T::default(); dim.width * dim.height] 
        }
    }

}

#[derive(Debug)]
pub struct Dimension {
    pub width: usize,
    pub height: usize
}

#[derive(Debug)]
pub struct MineField<'a> {

    dim: &'a Dimension,

    mines: Field<'a, bool>,
    shadow: Field<'a, bool>,
    num: Field<'a, u8>
}

impl <'a> MineField<'a> {

    pub fn new(dim: &'a Dimension) -> MineField<'a> {

        let mines = Field::new(&dim);
        let shadow = Field::new(&dim);
        let num = Field::new(&dim);

        MineField {
            dim: dim,
            mines: mines,
            shadow: shadow,
            num: num
        }
    }

    pub fn init(&mut self, num_mines: u32) {

        let width_range =         

    }

}




