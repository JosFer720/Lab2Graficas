mod framebuffer;
use framebuffer::*;

use raylib::prelude::*;
use std::time::{Duration, Instant};

fn main() {
    const SCALE: i32 = 6;
    const SCREEN_WIDTH: i32 = (WIDTH as i32) * SCALE;
    const SCREEN_HEIGHT: i32 = (HEIGHT as i32) * SCALE;
    const UPDATE_DELAY: Duration = Duration::from_millis(100);

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Conway's Game of Life")
        .build();

    rl.set_target_fps(60);

    let mut fb = Framebuffer::new();


    insert_glider(&mut fb, 2, 2);

    insert_blinker(&mut fb, 50, 50);

    insert_beacon(&mut fb, 5, 80);

    insert_lwss(&mut fb, 80, 5);

    insert_pulsar(&mut fb, 70, 40);

    insert_toad(&mut fb, 20, 50);

    insert_block(&mut fb, 10, 10);

    insert_beehive(&mut fb, 20, 10);

    insert_loaf(&mut fb, 30, 10);

    insert_boat(&mut fb, 40, 10);

    insert_tub(&mut fb, 15, 20);

    insert_pentadecathlon(&mut fb, 30, 70);

    insert_diehard(&mut fb, 60, 20);

    insert_acorn(&mut fb, 45, 30);

    insert_rpentomino(&mut fb, 85, 85);

    let mut last_update = Instant::now();

    while !rl.window_should_close() {
        if last_update.elapsed() >= UPDATE_DELAY {
            fb = fb.next_generation();
            last_update = Instant::now();
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if fb.cells[y][x] {
                    d.draw_rectangle(
                        (x as i32) * SCALE,
                        (y as i32) * SCALE,
                        SCALE,
                        SCALE,
                        Color::WHITE,
                    );
                }
            }
        }
    }
}


fn insert_glider(fb: &mut Framebuffer, x: usize, y: usize) {
    fb.set_alive(x + 1, y);
    fb.set_alive(x + 2, y + 1);
    fb.set_alive(x, y + 2);
    fb.set_alive(x + 1, y + 2);
    fb.set_alive(x + 2, y + 2);
}

fn insert_blinker(fb: &mut Framebuffer, x: usize, y: usize) {
    fb.set_alive(x, y);
    fb.set_alive(x + 1, y);
    fb.set_alive(x + 2, y);
}

fn insert_beacon(fb: &mut Framebuffer, x: usize, y: usize) {
    fb.set_alive(x, y);
    fb.set_alive(x + 1, y);
    fb.set_alive(x, y + 1);
    fb.set_alive(x + 3, y + 2);
    fb.set_alive(x + 2, y + 3);
    fb.set_alive(x + 3, y + 3);
}

fn insert_lwss(fb: &mut Framebuffer, x: usize, y: usize) {
    fb.set_alive(x + 1, y);
    fb.set_alive(x + 4, y);
    fb.set_alive(x, y + 1);
    fb.set_alive(x, y + 2);
    fb.set_alive(x + 4, y + 2);
    fb.set_alive(x + 1, y + 3);
    fb.set_alive(x + 2, y + 3);
    fb.set_alive(x + 3, y + 3);
    fb.set_alive(x + 4, y + 3);
}

fn insert_pulsar(fb: &mut Framebuffer, x: usize, y: usize) {
    let offsets = [
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
    ];

    for (dx, dy) in offsets {
        fb.set_alive(x + dx, y + dy);
    }
}

fn insert_toad(fb: &mut Framebuffer, x: usize, y: usize) {
    fb.set_alive(x + 1, y);
    fb.set_alive(x + 2, y);
    fb.set_alive(x + 3, y);
    fb.set_alive(x, y + 1);
    fb.set_alive(x + 1, y + 1);
    fb.set_alive(x + 2, y + 1);
}

fn insert_block(fb: &mut Framebuffer, x: usize, y: usize) {
    fb.set_alive(x, y);
    fb.set_alive(x + 1, y);
    fb.set_alive(x, y + 1);
    fb.set_alive(x + 1, y + 1);
}

fn insert_beehive(fb: &mut Framebuffer, x: usize, y: usize) {
    fb.set_alive(x + 1, y);
    fb.set_alive(x + 2, y);
    fb.set_alive(x, y + 1);
    fb.set_alive(x + 3, y + 1);
    fb.set_alive(x + 1, y + 2);
    fb.set_alive(x + 2, y + 2);
}

fn insert_loaf(fb: &mut Framebuffer, x: usize, y: usize) {
    fb.set_alive(x + 1, y);
    fb.set_alive(x + 2, y);
    fb.set_alive(x, y + 1);
    fb.set_alive(x + 3, y + 1);
    fb.set_alive(x + 1, y + 2);
    fb.set_alive(x + 3, y + 2);
    fb.set_alive(x + 2, y + 3);
}

fn insert_boat(fb: &mut Framebuffer, x: usize, y: usize) {
    fb.set_alive(x, y);
    fb.set_alive(x + 1, y);
    fb.set_alive(x, y + 1);
    fb.set_alive(x + 2, y + 1);
    fb.set_alive(x + 1, y + 2);
}

fn insert_tub(fb: &mut Framebuffer, x: usize, y: usize) {
    fb.set_alive(x + 1, y);
    fb.set_alive(x, y + 1);
    fb.set_alive(x + 2, y + 1);
    fb.set_alive(x + 1, y + 2);
}

fn insert_pentadecathlon(fb: &mut Framebuffer, x: usize, y: usize) {
    fb.set_alive(x + 1, y);
    fb.set_alive(x + 1, y + 1);
    fb.set_alive(x, y + 2);
    fb.set_alive(x + 2, y + 2);
    fb.set_alive(x + 1, y + 3);
    fb.set_alive(x + 1, y + 4);
    fb.set_alive(x + 1, y + 5);
    fb.set_alive(x + 1, y + 6);
    fb.set_alive(x, y + 7);
    fb.set_alive(x + 2, y + 7);
    fb.set_alive(x + 1, y + 8);
    fb.set_alive(x + 1, y + 9);
}

fn insert_diehard(fb: &mut Framebuffer, x: usize, y: usize) {
    fb.set_alive(x + 6, y);
    fb.set_alive(x, y + 1);
    fb.set_alive(x + 1, y + 1);
    fb.set_alive(x + 1, y + 2);
    fb.set_alive(x + 5, y + 2);
    fb.set_alive(x + 6, y + 2);
    fb.set_alive(x + 7, y + 2);
}

fn insert_acorn(fb: &mut Framebuffer, x: usize, y: usize) {
    fb.set_alive(x + 1, y);
    fb.set_alive(x + 3, y + 1);
    fb.set_alive(x, y + 2);
    fb.set_alive(x + 1, y + 2);
    fb.set_alive(x + 4, y + 2);
    fb.set_alive(x + 5, y + 2);
    fb.set_alive(x + 6, y + 2);
}

fn insert_rpentomino(fb: &mut Framebuffer, x: usize, y: usize) {
    fb.set_alive(x + 1, y);
    fb.set_alive(x + 2, y);
    fb.set_alive(x, y + 1);
    fb.set_alive(x + 1, y + 1);
    fb.set_alive(x + 1, y + 2);
}