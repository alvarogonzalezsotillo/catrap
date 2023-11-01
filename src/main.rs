// mod catrap;

mod amazonas;

fn main() {
    //catrap::level_80();

    let board = amazonas::find_board_minimize_trees(8);
    board.unwrap().dump_stdout("");
}
