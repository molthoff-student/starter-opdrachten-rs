use chrono::{DateTime, Local, Timelike};
use imgui::{ImColor32, Ui};
use std::f32::consts::PI;

const HOUR_MARKER: f32 = 0.50;
const MINUTE_MARKER: f32 = 0.80;
const SECOND_MARKER: f32 = 0.95;

pub fn draw_analog_clock(ui: &&mut Ui, region_avail: [f32; 2], current_date_time: DateTime<Local>) {
    let cursor_pos = ui.cursor_pos();

    let clock_region_avail = [
        region_avail[0] / 2.0, // Width / X
        region_avail[1] / 2.0, // Height / Y
    ];

    let clock_pos = [
        cursor_pos[0] + clock_region_avail[0],
        cursor_pos[1] + clock_region_avail[1],
    ];

    let clock_x = clock_pos[0];
    let clock_y = clock_pos[1];

    let radius = *clock_region_avail
        .iter()
        .min_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();

    if radius < 40.0 {
        return;
    }

    let draw = ui.get_window_draw_list();

    draw.add_circle(clock_pos, radius, ImColor32::BLACK)
        .filled(true)
        .build();

    draw.add_circle(clock_pos, radius, ImColor32::WHITE)
        .thickness(2.0)
        .build();

    let hours = (current_date_time.hour() % 12) as f32;
    let minutes = current_date_time.minute() as f32;
    let seconds = current_date_time.second() as f32;

    let minute_mark = minutes / 60.0;

    let hour_angle = (hours + minute_mark) / 12.0 * 2.0 * PI - PI / 2.0;
    let minute_angle = minute_mark * 2.0 * PI - PI / 2.0;
    let second_angle = seconds / 60.0 * 2.0 * PI - PI / 2.0;

    draw.add_line(
        clock_pos,
        [
            clock_x + hour_angle.cos() * radius * HOUR_MARKER,
            clock_y + hour_angle.sin() * radius * HOUR_MARKER,
        ],
        ImColor32::WHITE,
    )
    .thickness(4.0)
    .build();

    draw.add_line(
        clock_pos,
        [
            clock_x + minute_angle.cos() * radius * MINUTE_MARKER,
            clock_y + minute_angle.sin() * radius * MINUTE_MARKER,
        ],
        ImColor32::WHITE,
    )
    .thickness(3.0)
    .build();

    draw.add_line(
        clock_pos,
        [
            clock_x + second_angle.cos() * radius * SECOND_MARKER,
            clock_y + second_angle.sin() * radius * SECOND_MARKER,
        ],
        ImColor32::from_rgb(200, 50, 50),
    )
    .thickness(2.0)
    .build();
}
