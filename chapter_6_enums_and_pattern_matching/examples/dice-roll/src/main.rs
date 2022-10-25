fn main() {
    handle_play(3);
    handle_play(7);
    handle_play(9);
}

fn handle_play(dice_number: u32) {
    match dice_number {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}

fn add_fancy_hat() {
    println!("You've just earned a new fancy hat!");
}

fn remove_fancy_hat() {
    println!("You've just had your fancy hat removed!");
}

fn reroll() {
    println!("Re-rolling the dice");
}
