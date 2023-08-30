use chrono::{Duration, NaiveTime};
use imgui::{sys::ImGuiKey_Space, Drag, StyleColor, StyleVar};

const DEBUG_DRAW: bool = false;
macro_rules! debug {
    ($st: stmt;) => {
        if DEBUG_DRAW {
            $st
        };
    };
}

pub fn hello(ui: &imgui::Ui) {
    if let Some(_token) = ui.window("imgui_timeline_rs").begin() {
        ui.text("imgui_timeline_rs Version: 0.0.1");
    }
}

pub type Point = cgmath::Vector2<f32>;

pub struct Timeline {
    window_ident: String,
    name: String,
    time: chrono::NaiveTime,
    left_time: chrono::NaiveTime,
    tracks: Vec<Box<dyn Track>>,

    time_scale: f32,
    playback_speed: f32,
    playing: bool,
}

impl Timeline {
    pub fn new(name: impl Into<String>) -> Self {
        let name: String = name.into();
        Self {
            window_ident: format!("{0}##imgui_timeline_rs_timeline_{0}", name),
            name,
            time_scale: 0.4,
            time: NaiveTime::parse_from_str("00:00:00.0", "%H:%M:%S%.f").expect("WHy!!!!"),
            left_time: NaiveTime::parse_from_str("00:00:02.0", "%H:%M:%S%.f").expect("WHy!!!!"),
            playback_speed: 1.0,
            tracks: vec![
                Box::new(KeyFrameTrack {
                    name: "Test".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Test 2".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Transform".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Rotation4".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Rotation3".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Rotation1".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Rotation2".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Rotation".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Rotation5".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Rotation6".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Rotation7".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Rotation8".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Rotation9".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Rotationa".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Rotations".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Rotationg".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Rotationgg".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Dummy 1".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Dummy 2".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Dummy 3".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Dummy 4".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Dummy 5".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Dummy 6".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Dummy 7".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Dummy 8".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Dummy 9".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Dummy 10".to_string(),
                }),
                Box::new(KeyFrameTrack {
                    name: "Dummy 11".to_string(),
                }),
            ],

            playing: false,
        }
    }
}

impl Timeline {
    pub fn draw(&mut self, ui: &imgui::Ui, dt: f32) {
        if self.playing {
            self.time = self
                .time
                .overflowing_add_signed(Duration::milliseconds(
                    (dt * self.playback_speed * 1000.0) as i64,
                ))
                .0;
        }

        let color_frame_bg = ui.style_color(StyleColor::FrameBg);
        let color_border = ui.style_color(StyleColor::Border);

        let window_padding = ui.push_style_var(StyleVar::WindowPadding([0.0, 0.0]));
        let frame_border_size = ui.push_style_var(StyleVar::FrameBorderSize(0.0));

        if let Some(_token) = ui
            .window(&self.window_ident)
            .scroll_bar(false)
            .scrollable(false)
            .size([200.0, 200.0], imgui::Condition::FirstUseEver)
            .size_constraints([200.0, 200.0], [f32::INFINITY; 2])
            .begin()
        {
            let wdl = ui.get_window_draw_list();
            let top_left: Point = ui.window_pos().into();
            let focused =
                ui.is_window_focused_with_flags(imgui::WindowFocusedFlags::ROOT_AND_CHILD_WINDOWS);

            let height = ui.text_line_height_with_spacing();
            let item_spacing = ui.push_style_var(StyleVar::ItemSpacing([4.0, 0.0]));

            self.draw_toolbar(ui, height, &wdl, color_frame_bg, color_border);

            self.draw_editor(ui, height, &wdl, focused);

            self.draw_footer(ui, wdl, color_border);

            item_spacing.pop();
        }
        frame_border_size.pop();
        window_padding.pop();
    }

    fn draw_editor(
        &mut self,
        ui: &imgui::Ui,
        height: f32,
        wdl: &imgui::DrawListMut<'_>,
        focused: bool,
    ) {
        const ITEM_SPACING: f32 = 2.0;
        if let Some(_child) = ui
            .child_window("editor")
            .scroll_bar(false)
            .scrollable(false)
            .size([0.0, -(height + ITEM_SPACING)])
            .begin()
        {
            let top_left: Point = ui.window_pos().into();
            let region_max: Point = ui.content_region_max().into();
            debug! {
                        wdl.add_rect(
                            (top_left + Point::new(1.0, 1.0)).array(),
                            (top_left + region_max - Point::new(1.0, 1.0)).array(),
                            [0.0, 1.0, 0.0],
                        )
                        .thickness(1.0)
                        .build();
            }
            // ui.text("Editor")

            let cell_padding = ui.push_style_var(StyleVar::CellPadding([0.0; 2]));
            if let Some(_table) = ui.begin_table_with_flags(
                "track_table",
                2,
                imgui::TableFlags::PRECISE_WIDTHS
                    | imgui::TableFlags::RESIZABLE
                    | imgui::TableFlags::BORDERS_H
                    | imgui::TableFlags::BORDERS_V
                    | imgui::TableFlags::SCROLL_Y,
            ) {
                if focused && ui.is_key_index_pressed_no_repeat(ImGuiKey_Space) {
                    self.playing = !self.playing;
                }

                ui.table_setup_scroll_freeze(2, 1);
                ui.table_setup_column("##0");
                ui.table_setup_column("##1");

                const FIRST_HEIGHT: f32 = 35.0;
                ui.table_next_row();
                ui.table_set_column_index(0);
                ui.set_window_font_scale(1.25); //TODO: Replace with bigger font.
                let time = self.time.format("%H:%M:%S%.3f").to_string();
                let size: Point = ui.calc_text_size(&time).into();
                let max: Point = ui.content_region_max().into();
                ui.set_cursor_pos([
                    (max.x - size.x) / 2.0,
                    ui.cursor_pos()[1] + (FIRST_HEIGHT - ui.text_line_height_with_spacing()) / 2.0,
                ]);
                ui.text(time);
                ui.set_window_font_scale(1.0);

                wdl.add_line(
                    [top_left.x, top_left.y + FIRST_HEIGHT],
                    [top_left.x + region_max.x, top_left.y + FIRST_HEIGHT],
                    [7.0, 7.0, 7.0],
                )
                .thickness(1.5)
                .build();

                ui.table_set_column_index(1);

                if let Some(_t) = ui
                    .child_window("timeline")
                    .scroll_bar(false)
                    .scrollable(false)
                    .size([0.0, FIRST_HEIGHT])
                    .begin()
                {
                    let top_left: Point = ui.window_pos().into();
                    let outer_region_max = region_max;
                    let region_max: Point = ui.content_region_max().into();

                    const SECOND_WIDTH: f32 = 250.0;
                    let second_width = SECOND_WIDTH * self.time_scale;

                    //Input
                    if focused && ui.is_window_hovered() {
                        if ui.is_mouse_clicked(imgui::MouseButton::Left) {
                            let mut mouse_pos = ui.io().mouse_pos[0];
                            mouse_pos -= top_left.x;
                            mouse_pos /= second_width;
                            self.time = self
                                .left_time
                                .overflowing_add_signed(Duration::milliseconds(
                                    (mouse_pos * 1000.0) as i64,
                                ))
                                .0;
                        }
                    }
                    debug! {
                                wdl.add_rect(
                                    (top_left + Point::new(1.0, 1.0)).array(),
                                    (top_left + region_max - Point::new(1.0, 1.0)).array(),
                                    [1.0, 1.0, 0.0],
                                )
                                .thickness(1.0)
                                .build();
                    }

                    let width = region_max.x;
                    wdl.with_clip_rect(top_left.array(), (top_left + region_max).array(), || {
                        ui.set_window_font_scale(0.9); //TODO: Replace with smaller font
                        let mut x = 0.0;
                        while x <= width {
                            wdl.add_line(
                                [top_left.x + x, top_left.y + region_max.y],
                                [top_left.x + x, top_left.y + region_max.y * 0.333],
                                [1.0; 3],
                            )
                            .thickness(1.0)
                            .build();

                            let between_steps = 10;
                            for i in 1..between_steps {
                                wdl.add_line(
                                    [
                                        top_left.x
                                            + x
                                            + second_width / between_steps as f32 * i as f32,
                                        top_left.y + region_max.y,
                                    ],
                                    [
                                        top_left.x
                                            + x
                                            + second_width / between_steps as f32 * i as f32,
                                        top_left.y + region_max.y * 0.666,
                                    ],
                                    [0.5; 3],
                                )
                                .thickness(1.0)
                                .build();
                            }

                            wdl.add_text(
                                [top_left.x + x + 5.0, top_left.y + region_max.y * 0.1],
                                [1.0; 3],
                                (self
                                    .left_time
                                    .overflowing_add_signed(Duration::milliseconds(
                                        (x / second_width * 1000.0) as i64,
                                    )))
                                .0
                                .format("%H:%M:%S")
                                .to_string(),
                            );
                            x += second_width;
                        }
                        ui.set_window_font_scale(1.0);
                    });

                    self.draw_track_head(top_left, second_width, wdl, region_max, outer_region_max);
                }

                for track in &mut self.tracks {
                    ui.table_next_row();
                    ui.table_set_column_index(0);
                    draw_track(track, ui);
                }
            }

            cell_padding.pop();
        }
    }

    fn draw_track_head(
        &mut self,
        top_left: cgmath::Vector2<f32>,
        second_width: f32,
        wdl: &imgui::DrawListMut<'_>,
        region_max: cgmath::Vector2<f32>,
        outer_region_max: cgmath::Vector2<f32>,
    ) {
        let track_head_center = top_left.x
            + ((self.time - self.left_time).num_milliseconds() as f32 / 1000.0) * second_width;

        if track_head_center >= top_left.x {
            wdl.add_polyline(
                vec![
                    [track_head_center, top_left.y + region_max.y],
                    [track_head_center - 5.0, top_left.y + region_max.y * 0.8],
                    [track_head_center - 5.0, top_left.y + region_max.y * 0.6],
                    [track_head_center + 5.0, top_left.y + region_max.y * 0.6],
                    [track_head_center + 5.0, top_left.y + region_max.y * 0.8],
                ],
                [1.0, 0.0, 0.0],
            )
            .filled(true)
            .thickness(2.0)
            .build();

            wdl.add_line(
                [track_head_center, top_left.y + region_max.y * 0.7],
                [track_head_center, top_left.y + outer_region_max.y],
                [1.0, 0.0, 0.0],
            )
            .thickness(2.0)
            .build();
        }
    }

    fn draw_toolbar(
        &mut self,
        ui: &imgui::Ui,
        height: f32,
        wdl: &imgui::DrawListMut<'_>,
        color_frame_bg: [f32; 4],
        color_border: [f32; 4],
    ) {
        if let Some(_child) = ui
            .child_window("toolbar")
            .scroll_bar(false)
            .scrollable(false)
            .size([0.0, height])
            .begin()
        {
            let top_left: Point = ui.window_pos().into();
            let region_max: Point = ui.content_region_max().into();
            debug! {
                        wdl.add_rect(
                            (top_left + Point::new(1.0, 1.0)).array(),
                            (top_left + region_max - Point::new(1.0, 1.0)).array(),
                            [1.0, 0.0, 0.0],
                        )
                        .thickness(1.0)
                        .build();
            };

            //BG
            wdl.add_rect(
                top_left.array(),
                (top_left + region_max).array(),
                color_frame_bg,
            )
            .filled(true)
            .build();
            wdl.add_line(
                [top_left.x, top_left.y + region_max.y],
                [top_left.x + region_max.x, top_left.y + region_max.y],
                color_border,
            )
            .thickness(1.0)
            .build();

            ui.set_cursor_pos([2.0, (region_max.y - height) / 2.0 + 2.0]);

            // Toolbar Text
            ui.text("Toolbar: ");
            ui.same_line();
            // ui.button_with_size("Test", [0.0, height - 4.0]);
            // ui.same_line();

            //Scale Input
            ui.text("Scale:");
            ui.same_line();
            ui.set_next_item_width(50.0);
            Drag::new("##scale")
                .speed(0.05)
                .range(0.01, 10.0)
                .display_format("%.2f")
                .build(ui, &mut self.time_scale);

            ui.same_line();

            //SPeed Input
            ui.text("Speed:");
            ui.same_line();
            ui.set_next_item_width(50.0);
            Drag::new("##speed")
                .speed(0.01)
                .range(0.1, 5.0)
                .display_format("%.2f")
                .build(ui, &mut self.playback_speed);
        };
    }

    fn draw_footer(&mut self, ui: &imgui::Ui, wdl: imgui::DrawListMut<'_>, color_border: [f32; 4]) {
        if let Some(_child) = ui
            .child_window("footer")
            .scroll_bar(false)
            .scrollable(false)
            .size([0.0, 0.0])
            .begin()
        {
            let top_left: Point = ui.window_pos().into();
            let region_max: Point = ui.content_region_max().into();
            debug! {
                        wdl.add_rect(
                            (top_left + Point::new(1.0, 1.0)).array(),
                            (top_left + region_max - Point::new(1.0, 1.0)).array(),
                            [0.0, 0.0, 1.0],
                        )
                        .thickness(1.0)
                        .build();
            }

            wdl.add_line(
                [top_left.x, top_left.y],
                [top_left.x + region_max.x, top_left.y],
                color_border,
            )
            .thickness(1.0)
            .build();

            ui.text("Footer")
        }
    }
}

trait AsArray {
    type T;
    fn array(&self) -> [Self::T; 2];
}

impl AsArray for Point {
    type T = f32;

    fn array(&self) -> [Self::T; 2] {
        [self.x, self.y]
    }
}

pub trait Track {
    fn draw_head(&mut self, ui: &imgui::Ui);
    fn head_config(&mut self) -> (String, f32);
}

pub struct KeyFrameTrack {
    name: String,
}

impl KeyFrameTrack {
    const TRACK_HEIGHT: f32 = 30.0;
}

impl Track for KeyFrameTrack {
    fn draw_head(&mut self, ui: &imgui::Ui) {
        let max: Point = ui.content_region_max().into();
        let size: Point = ui.calc_text_size(&self.name).into();
        ui.set_cursor_pos([
            (max.x - size.x) / 2.0,
            ui.cursor_pos()[1] + (Self::TRACK_HEIGHT - ui.text_line_height_with_spacing()) / 2.0,
        ]);
        ui.text(&self.name);
    }

    fn head_config(&mut self) -> (String, f32) {
        (
            format!("keyframe_track_head_{}", self.name),
            Self::TRACK_HEIGHT,
        )
    }
}

fn draw_track(track: &mut Box<dyn Track>, ui: &imgui::Ui) {
    let config = track.head_config();
    if let Some(_c) = ui
        .child_window(config.0)
        .scroll_bar(false)
        .scrollable(false)
        .size([0.0, config.1])
        .begin()
    {
        track.draw_head(ui);
    }
    ui.next_column();
}
