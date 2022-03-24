# Game for programmers

**The goal of this project is to create a simple game for programmers.**

## The essence of the game

Each player has his own character that he can control using a code.
Players connect to the server and join the game with other players. Several players are on the same map. The player's goal is to remain the only player on the map.

Players can send multiple commands to the server to control their character. (For example, moving, attacking, reloading weapons, etc.)


All that is required is to connect to the web socket and send messages in the format from `jsons` direction.

### Steps
1. Connect to server
2. Sign in or sign up
3. Wait start game
4. Play game
5. Go to step number 3 