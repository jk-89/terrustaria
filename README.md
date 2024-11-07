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
- Generating world maps - we implemented random map generation, spawning resources, and spawning caves. We also created a wall map and prepared a space for buildings.
- Different tiles types.
- Tiles are initially covered, player uncover tiles by moving in their direction.
- A player and an option to move - after hours spent with colliders, we managed to add them to map and player, they do collide.
- Highlighting the tiles within player range.
- Destroying tiles within player range.
- After launching the game, using A, D, and space for jumping, you can steer our cool player, and using Z, X you can move the camera (option for debug).
- Camera follows the player.

## How to run the game
- Just `cargo run`.
- In case you need debug information:  
  `cargo build --features "debug" && cargo run --features "debug"`  
  Note that it may lead to decrease in app's performance.
- To steer our cool player use **A, D, SPACE**.
- Camera movement with Arrows and Z, X for zoom.
