# Terrustaria

## Authors
- Tomasz Głąb ([@Toomimi](https://github.com/Toomimi) on GitHub)
- Jan Kwiatkowski ([@jk-89](https://github.com/jk-89) on GitHub)

## Description
Terrustaria is a simplified Rust implementation of well known game - Terraria.

## Libraries
- Bevy
- [bevy_ecs_tilemap](https://github.com/StarArawn/bevy_ecs_tilemap) library
- Serde for serialization

## What we implemented
- Creating block types.
- Destroying blocks within player range (with animation!).
- Generating world maps - we implemented random map generation, spawning resources, and spawning caves. We also created a wall map and prepared a space for building in the next part.
- Tiles are initially covered, player uncover tiles by moving in their direction.
- Adding a player and an option to move - after hours spent with colliders, we managed to add them to map and player, they do collide.
- Player is able to mine now - we implemented highlighting the tiles, the next steps are: highlight them in range of a player, and destroy tiles that are in the range of the player.
- After launching the game, using A, D, and space for jumping, you can steer our cool player, and using Z, X you can move the camera (option for debug). Right now colliders projection can be seen, this is also a debug option.
- Camera follows the player.

### How to run the game
- Just `cargo run`
- In case you need debug information:  
  `cargo build --features "debug" && cargo run --features "debug"`  
  Note that it may lead to decrease in app's performance.
- Now to steer our even cooler player use **A, D, SPACE** 
- We left 2 debug options:
  - colliders projection can be seen
  - camera movement with Arrows and Z, X for zoom 
