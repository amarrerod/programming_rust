
use std::sync::Mutex;

type PlayerId = u32;
const GAME_SIZE : usize = 8;
type WaitingList = Vec<PlayerId>;


pub struct UltimateGame {
    pub waiting_list : Mutex<WaitingList>,
}

impl UltimateGame {
    pub fn join_waiting_list(&self, player: PlayerId) {
        let mut guard = self.waiting_list.lock().unwrap();
        guard.push(player);
        if guard.len() == GAME_SIZE {
            let players = guard.split_off(0);
            self.start_game(&players);
        }
    }

    fn start_game(&self, players: &Vec<u32>) {
        println!("Starting the game with the following players: {:?}", players);
    }
}