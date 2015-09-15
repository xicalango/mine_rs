extern crate mine_rs;

fn main() {

    let dim = mine_rs::Dimension { width: 10, height: 10 };
    

    let mineField = mine_rs::MineField::new(&dim);

    let game = mine_rs::Game::new(mineField);



}
