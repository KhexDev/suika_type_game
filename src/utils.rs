use bevy::prelude::*;

use crate::fruits::FruitType;

pub fn get_fruits(fruit: FruitType) -> (f32, Color) {
    let ball_size: f32;
    let fruit_color: Color;
    match fruit {
        FruitType::Cherry => {
            ball_size = 0.10;
            fruit_color = Color::rgb_u8(139, 0, 0);
        },
        FruitType::Strawberry => {
            ball_size = 0.20;
            fruit_color = Color::RED;
        },
        FruitType::Grapes => {
            ball_size = 0.3;
            fruit_color = Color::GREEN;
        }
        FruitType::Dekopon => {
            ball_size = 0.35;
            fruit_color = Color::ORANGE;
        },
        FruitType::Persimon => {
            ball_size = 0.55;
            fruit_color = Color::ORANGE;
        },
        FruitType::Apple => {
            ball_size = 0.6;
            fruit_color = Color::RED;
        }
        FruitType::Pear => {
            ball_size = 0.7;
            fruit_color = Color::YELLOW_GREEN;
        }
        FruitType::Peach => {
            ball_size = 0.8;
            fruit_color = Color::ORANGE_RED;
        }
        FruitType::Pineapple => {
            ball_size = 0.9;
            fruit_color = Color::YELLOW;
        }
        FruitType::Melon => {
            ball_size = 1.0;
            fruit_color = Color::SEA_GREEN
        }
        FruitType::Watermelon => {
            ball_size = 1.2;
            fruit_color = Color::DARK_GREEN;
        }
    }

    (ball_size, fruit_color)
}

pub fn get_score_from_fruits(fruit_type: FruitType) -> i32 {
    let score: i32;

    match fruit_type {
        FruitType::Cherry => score = 50,
        FruitType::Strawberry => score = 75,
        FruitType::Grapes => score = 100,
        FruitType::Dekopon => score = 150,
        FruitType::Persimon => score = 175,
        FruitType::Apple => score = 200,
        FruitType::Pear => score = 250,
        FruitType::Peach => score = 300,
        FruitType::Pineapple => score = 350,
        FruitType::Melon => score = 400,
        FruitType::Watermelon => score = 450,
    }

    score
}