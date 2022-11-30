/*
* Copyright (C) 2022 Rastislav Kish
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, version 3.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use std::thread;
use std::time::Duration;

use clap::Parser;
use mouse_rs::Mouse;


/// Mouse location keeper
#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    /// The x coordinate of the target location
    #[arg(short='x', long, default_value_t=100)]
    target_x: i32,
    /// The y coordinate of the target location
    #[arg(short='y', long, default_value_t=100)]
    target_y: i32,
    /// The count of pixels the mouse can diverge from the target position, horizontally or vertically
    #[arg(short='d', long, default_value_t=100)]
    max_delta: i32,
    }

fn main() {
    let args=Args::parse();

    let mouse=Mouse::new();

    let Args {target_x, target_y, max_delta}=args;

    mouse.move_to(target_x, target_y).unwrap();

    loop {
        if let Ok(point)=mouse.get_position() {
            if (point.x-target_x).abs()>max_delta || (point.y-target_y).abs()>max_delta {
                thread::sleep(Duration::from_secs(10));
                mouse.move_to(target_x, target_y).unwrap();
                }
            }

        thread::sleep(Duration::from_secs(10));
        }
    }
