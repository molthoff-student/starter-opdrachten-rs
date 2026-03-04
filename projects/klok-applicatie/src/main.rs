mod app;
mod window;

use app::draw_analog_clock;
use chrono::Local;
use window::*;

use imgui::{ChildFlags, Condition, SliderFlags, WindowFlags};

use crate::app::AlarmClock;

const WIN_NAME: &str = "Time";

fn main() {
    create_window(
        WIN_NAME,
        // This application doesn't require a defined startup closure.
        |_, _, _| {},
        |run, ui, app_data| {
            let window_flags = WindowFlags::NO_TITLE_BAR
                | WindowFlags::NO_RESIZE
                | WindowFlags::NO_MOVE
                | WindowFlags::NO_COLLAPSE
                | WindowFlags::NO_DECORATION;

            let display_size = ui.io().display_size;

            let win_main = ui
                .window("MAIN_WINDOW")
                .opened(run)
                .position([0.0, 0.0], Condition::Always)
                .flags(window_flags)
                .size(display_size, Condition::Always);

            win_main.build(|| {
                let Some(utility_bar) = ui.tab_bar("UTILITY_BAR") else {
                    println!("Failed to construct utility bar.");
                    return;
                };

                if let Some(item) = ui.tab_item("Clock") {
                    let current_date_time = Local::now();

                    let font_size = app_data.settings.font_scale;
                    ui.set_window_font_scale(font_size * 1.25);

                    let date = current_date_time.date_naive();
                    ui.text(format!("Date: {}", date));

                    let time = current_date_time.format("%H:%M:%S");
                    ui.text(format!("Time: {}", time));

                    ui.set_window_font_scale(font_size);

                    let region_avail = ui.content_region_avail();

                    let can_render_clock = region_avail.iter().all(|len| len.is_normal());

                    if can_render_clock {
                        draw_analog_clock(&ui, region_avail, current_date_time);
                    }

                    item.end();
                }

                if let Some(item) = ui.tab_item("Alarms") {
                    let alarm_clock = &mut app_data.alarm_clock;

                    let cursor_pos = ui.cursor_pos();
                    let cursor_x = cursor_pos[0];
                    let cursor_y = cursor_pos[1];

                    let region_avail = ui.content_region_avail();
                    let width = 0.25 * region_avail[0];
                    let height = region_avail[1];

                    let man_height = 0.25 * height;
                    let bar_height = 0.75 * height;

                    ui.set_cursor_pos(cursor_pos);
                    ui.child_window("ALARM_MANAGER")
                        .size([width, man_height])
                        .child_flags(ChildFlags::BORDERS)
                        .build(|| {
                            if ui.button("Add new alarm") {
                                let index = alarm_clock.list.len();
                                let alarm_name = String::from(format!("Alarm {}", index));
                                let alarm = AlarmClock::new(alarm_name);
                                alarm_clock.list.push(alarm);
                                alarm_clock.selected = Some(index);
                            }
                            if ui.button("Save all alarms") {
                                alarm_clock.save().expect("Failed to save alarms");
                            }
                        });

                    ui.set_cursor_pos([cursor_x, cursor_y + man_height]);
                    ui.child_window("ALARM_SCROLL_BAR")
                        .size([width, bar_height])
                        .child_flags(ChildFlags::BORDERS)
                        .build(|| {
                            for (index, alarm) in alarm_clock.list.iter().enumerate() {
                                let id_token = ui.push_id_usize(index);
                                if ui.selectable_config(alarm.name.as_str()).build() {
                                    alarm_clock.selected = Some(index);
                                }
                                ui.separator();
                                id_token.end();
                            }
                        });

                    ui.set_cursor_pos([cursor_x + width, cursor_y]);
                    ui.child_window("SELECTED_ALARM_ENTRY")
                        .size([0.0, height])
                        .child_flags(ChildFlags::BORDERS)
                        .build(|| {
                            if let Some(index) = alarm_clock.selected
                                && let Some(alarm) = alarm_clock.list.get_mut(index)
                            {
                                ui.input_text("##ALARM_NAME", &mut alarm.name).build();
                                if ui.button("Delete alarm") {
                                    alarm_clock.list.remove(index);
                                    let current_len = alarm_clock.list.len();
                                    if current_len == 0 {
                                        alarm_clock.selected = None;
                                    } else if current_len < index {
                                        alarm_clock.selected = Some(index - 1);
                                    }
                                }
                            } else {
                                ui.text("No alarm selected.");
                            }
                        });

                    item.end();
                }

                if let Some(item) = ui.tab_item("Settings") {
                    let settings = &mut app_data.settings;
                    ui.slider_config("Font scale", 0.01, 10.0)
                        .flags(SliderFlags::ALWAYS_CLAMP)
                        .build(&mut settings.font_scale);

                    if ui.is_item_deactivated_after_edit() {
                        ui.set_window_font_scale(settings.font_scale);
                    }

                    ui.checkbox("Auto save ", &mut settings.autosave);
                    ui.checkbox("Save settings when closing", &mut settings.exitsave);

                    item.end();
                }

                utility_bar.end();
            });
        },
    )
}
