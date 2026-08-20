// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:
// * Use a variable set to either true or false
// * Use an if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

enum PlayerState { Coming, Going }

fn main() {
    let mut player_state = PlayerState::Coming;
    print_message_for_player_state(player_state);
    player_state = PlayerState::Going;
    print_message_for_player_state(player_state);
}

fn print_message_for_player_state(player_state: PlayerState) {
    match player_state {
        PlayerState::Coming => println!("Hello"),
        PlayerState::Going => println!("Goodbye")
    }
}
