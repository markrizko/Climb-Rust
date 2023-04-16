use climb_rust::game::Game;

fn main() {
    let mut pflag = true;
    let mut gm = Game::new();
    while pflag {
        gm.welcome_screen();
        gm.run_game();
        if gm.pa {
            gm = Game::new();
        }
        else {
            pflag = false;
        }
    }
}
