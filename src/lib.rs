extern crate rand;

use std::rc::Rc;

#[derive(Debug)]
pub struct Field<T> {
    dim: Rc<Dimension>,
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

impl <T: Copy> Access2D<T> for Field<T> {

    fn get_at(&self, x: usize, y: usize) -> T {
        self.field[ get_index( &self.dim, x, y ) ]
    }

    fn set_at(&mut self, x: usize, y: usize, value: T) {
        self.field[ get_index( &self.dim, x, y ) ] = value;
    }
}

impl <T: Copy + Default> Field<T> {

    pub fn new(dim: Rc<Dimension>) -> Field<T> {
        let width = dim.width;
        let height = dim.height;
        Field {
            dim: dim,
            field: vec![T::default(); width * height] 
        }
    }

}

#[derive(Debug)]
pub struct Dimension {
    pub width: usize,
    pub height: usize
}

#[derive(Debug)]
pub struct MineField {

    dim: Rc<Dimension>,

    mines: Field<bool>,
    shadow: Field<bool>,
    num: Field<u8>
}

impl MineField {

    pub fn new(width: usize, height: usize) -> MineField {

        let dim = Rc::new(Dimension{ width: width, height: height });

        let mines = Field::new(dim.clone());
        let shadow = Field::new(dim.clone());
        let num = Field::new(dim.clone());

        MineField {
            dim: dim,
            mines: mines,
            shadow: shadow,
            num: num
        }
    }

}




