use std::time::Instant;
use iced::mouse::{self, Cursor};
use iced::widget::canvas::{self, Cache, Event, Geometry, Path, Program, Stroke, Text};
use iced::{Color, Point, Rectangle, Renderer, Size, Theme};

use blanc_core::flow::{FlowNode, FlowNodeType, StoryFlowGraph};
use blanc_core::scene::ActionBlock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowInteractionMode {
    Select,
    Connect,
    CutWire,
}

impl Default for FlowInteractionMode {
    fn default() -> Self {
        Self::Select
    }
}

#[derive(Debug, Default)]
pub struct FlowCanvasState {
    pub cache: Cache,
    pub is_panning: bool,
    pub pan_start: Option<Point>,
    pub dragging_node_id: Option<String>,
    pub drag_offset_x: f32,
    pub drag_offset_y: f32,
    pub connecting_pin: Option<(String, String)>, // (node_id, pin_id)
    pub current_mouse_pos: Option<Point>,
    pub last_click_time: Option<Instant>,
    pub last_click_node_id: Option<String>,
}

#[derive(Debug, Clone)]
pub enum FlowCanvasAction {
    SelectNode(String),
    OpenSceneInEditor(String),
    ConnectPins {
        from_node: String,
        from_pin: String,
        to_node: String,
        to_pin: String,
    },
    DisconnectPin {
        node_id: String,
        pin_id: String,
    },
    SetNodePosition {
        node_id: String,
        x: f32,
        y: f32,
    },
    PanCanvas {
        dx: f32,
        dy: f32,
    },
    ZoomDelta(f32),
    RemoveNode(String),
    ClearSelection,
    SetMode(FlowInteractionMode),
    FocusAll,
    AutoLayout,
    ToggleSnapGrid,
    ResetView,
}

pub struct FlowCanvasWidget<'a> {
    pub graph: &'a StoryFlowGraph,
    pub scenes: &'a [blanc_core::scene::Scene],
    pub selected_node_id: Option<&'a str>,
    pub mode: FlowInteractionMode,
    pub snap_grid: bool,
}

impl<'a> Program<FlowCanvasAction, Theme, Renderer> for FlowCanvasWidget<'a> {
    type State = FlowCanvasState;

    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (canvas::event::Status, Option<FlowCanvasAction>) {
        let cursor_position = match cursor.position_in(bounds) {
            Some(pos) => pos,
            None => {
                if let Event::Mouse(mouse::Event::ButtonReleased(_)) = event {
                    state.is_panning = false;
                    state.dragging_node_id = None;
                    state.connecting_pin = None;
                    state.cache.clear();
                }
                return (canvas::event::Status::Ignored, None);
            }
        };

        state.current_mouse_pos = Some(cursor_position);

        match event {
            Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }) => {
                match key.as_ref() {
                    iced::keyboard::Key::Character("v") | iced::keyboard::Key::Character("V") | iced::keyboard::Key::Character("1") => {
                        state.cache.clear();
                        return (canvas::event::Status::Captured, Some(FlowCanvasAction::SetMode(FlowInteractionMode::Select)));
                    }
                    iced::keyboard::Key::Character("c") | iced::keyboard::Key::Character("C") | iced::keyboard::Key::Character("2") => {
                        state.cache.clear();
                        return (canvas::event::Status::Captured, Some(FlowCanvasAction::SetMode(FlowInteractionMode::Connect)));
                    }
                    iced::keyboard::Key::Character("x") | iced::keyboard::Key::Character("X") | iced::keyboard::Key::Character("3") => {
                        state.cache.clear();
                        return (canvas::event::Status::Captured, Some(FlowCanvasAction::SetMode(FlowInteractionMode::CutWire)));
                    }
                    iced::keyboard::Key::Character("f") | iced::keyboard::Key::Character("F") => {
                        state.cache.clear();
                        return (canvas::event::Status::Captured, Some(FlowCanvasAction::FocusAll));
                    }
                    iced::keyboard::Key::Character("a") | iced::keyboard::Key::Character("A") => {
                        state.cache.clear();
                        return (canvas::event::Status::Captured, Some(FlowCanvasAction::AutoLayout));
                    }
                    iced::keyboard::Key::Character("s") | iced::keyboard::Key::Character("S") => {
                        state.cache.clear();
                        return (canvas::event::Status::Captured, Some(FlowCanvasAction::ToggleSnapGrid));
                    }
                    iced::keyboard::Key::Character("0") => {
                        state.cache.clear();
                        return (canvas::event::Status::Captured, Some(FlowCanvasAction::ResetView));
                    }
                    iced::keyboard::Key::Character("+") | iced::keyboard::Key::Character("=") => {
                        state.cache.clear();
                        return (canvas::event::Status::Captured, Some(FlowCanvasAction::ZoomDelta(0.15)));
                    }
                    iced::keyboard::Key::Character("-") => {
                        state.cache.clear();
                        return (canvas::event::Status::Captured, Some(FlowCanvasAction::ZoomDelta(-0.15)));
                    }
                    iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) => {
                        state.connecting_pin = None;
                        state.cache.clear();
                        return (canvas::event::Status::Captured, Some(FlowCanvasAction::ClearSelection));
                    }
                    _ => (canvas::event::Status::Ignored, None),
                }
            }

            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let zoom = self.graph.zoom;
                let pan_x = self.graph.pan_x;
                let pan_y = self.graph.pan_y;

                // SPECIAL MODE: CutWire
                if self.mode == FlowInteractionMode::CutWire {
                    // 1. Check if clicked an output pin to disconnect all wires from it
                    for node in &self.graph.nodes {
                        let nx = node.x * zoom + pan_x;
                        let ny = node.y * zoom + pan_y;
                        let nw = self.get_node_width(node) * zoom;
                        let pin_start_y = ny + 85.0 * zoom;

                        let output_pins = self.get_node_output_pins(node);
                        for (p_idx, (pin_id, _label)) in output_pins.iter().enumerate() {
                            let pin_x = nx + nw;
                            let pin_y = pin_start_y + (p_idx as f32 * 30.0) * zoom;
                            let dist = ((cursor_position.x - pin_x).powi(2) + (cursor_position.y - pin_y).powi(2)).sqrt();
                            if dist <= 16.0 * zoom {
                                state.cache.clear();
                                return (
                                    canvas::event::Status::Captured,
                                    Some(FlowCanvasAction::DisconnectPin {
                                        node_id: node.id.clone(),
                                        pin_id: pin_id.clone(),
                                    }),
                                );
                            }
                        }
                    }

                    // 2. Check if clicked along any wire curve
                    for conn in &self.graph.connections {
                        if let (Some(p1), Some(p2)) = (
                            self.get_pin_screen_pos(&conn.from_node_id, &conn.from_pin_id, true),
                            self.get_pin_screen_pos(&conn.to_node_id, &conn.to_pin_id, false),
                        ) {
                            let cx1 = p1.x + (p2.x - p1.x).abs().max(40.0) * 0.5;
                            let cx2 = p2.x - (p2.x - p1.x).abs().max(40.0) * 0.5;
                            for t_step in 0..=12 {
                                let t = t_step as f32 / 12.0;
                                let one_minus_t = 1.0 - t;
                                let bx = one_minus_t.powi(3) * p1.x
                                    + 3.0 * one_minus_t.powi(2) * t * cx1
                                    + 3.0 * one_minus_t * t.powi(2) * cx2
                                    + t.powi(3) * p2.x;
                                let by = one_minus_t.powi(3) * p1.y
                                    + 3.0 * one_minus_t.powi(2) * t * p1.y
                                    + 3.0 * one_minus_t * t.powi(2) * p2.y
                                    + t.powi(3) * p2.y;
                                let d = ((cursor_position.x - bx).powi(2) + (cursor_position.y - by).powi(2)).sqrt();
                                if d <= 14.0 {
                                    state.cache.clear();
                                    return (
                                        canvas::event::Status::Captured,
                                        Some(FlowCanvasAction::DisconnectPin {
                                            node_id: conn.from_node_id.clone(),
                                            pin_id: conn.from_pin_id.clone(),
                                        }),
                                    );
                                }
                            }
                        }
                    }
                }

                // 1. Check if clicked on an Output Pin
                for node in &self.graph.nodes {
                    let nx = node.x * zoom + pan_x;
                    let ny = node.y * zoom + pan_y;
                    let nw = self.get_node_width(node) * zoom;
                    let pin_start_y = ny + 85.0 * zoom;

                    let output_pins = self.get_node_output_pins(node);
                    for (p_idx, (pin_id, _label)) in output_pins.iter().enumerate() {
                        let pin_x = nx + nw;
                        let pin_y = pin_start_y + (p_idx as f32 * 30.0) * zoom;
                        let dist = ((cursor_position.x - pin_x).powi(2) + (cursor_position.y - pin_y).powi(2)).sqrt();
                        if dist <= 14.0 * zoom {
                            state.connecting_pin = Some((node.id.clone(), pin_id.clone()));
                            state.cache.clear();
                            return (canvas::event::Status::Captured, None);
                        }
                    }

                    // Input pin click
                    if self.node_has_input_pin(node) {
                        let pin_x = nx;
                        let pin_y = pin_start_y;
                        let dist = ((cursor_position.x - pin_x).powi(2) + (cursor_position.y - pin_y).powi(2)).sqrt();
                        if dist <= 14.0 * zoom {
                            if let Some((from_node, from_pin)) = state.connecting_pin.take() {
                                if from_node != node.id {
                                    state.cache.clear();
                                    return (
                                        canvas::event::Status::Captured,
                                        Some(FlowCanvasAction::ConnectPins {
                                            from_node,
                                            from_pin,
                                            to_node: node.id.clone(),
                                            to_pin: "in".to_string(),
                                        }),
                                    );
                                }
                            }
                        }
                    }
                }

                // 2. Check if clicked on Delete Button [X] of a node
                for node in &self.graph.nodes {
                    if node.id != "system_main_menu" {
                        let nx = node.x * zoom + pan_x;
                        let ny = node.y * zoom + pan_y;
                        let nw = self.get_node_width(node) * zoom;
                        let del_btn_x = nx + nw - 18.0 * zoom;
                        let del_btn_y = ny + 17.0 * zoom;
                        let dist = ((cursor_position.x - del_btn_x).powi(2) + (cursor_position.y - del_btn_y).powi(2)).sqrt();
                        if dist <= 14.0 * zoom {
                            state.cache.clear();
                            return (
                                canvas::event::Status::Captured,
                                Some(FlowCanvasAction::RemoveNode(node.id.clone())),
                            );
                        }
                    }
                }

                // 3. Check if clicked on a Node header or body
                for node in self.graph.nodes.iter().rev() {
                    let nx = node.x * zoom + pan_x;
                    let ny = node.y * zoom + pan_y;
                    let nw = self.get_node_width(node) * zoom;
                    let nh = self.get_node_height(node) * zoom;

                    if cursor_position.x >= nx
                        && cursor_position.x <= nx + nw
                        && cursor_position.y >= ny
                        && cursor_position.y <= ny + nh
                    {
                        // Check double click (within 350ms)
                        let now = Instant::now();
                        let is_double_click = if let (Some(last_id), Some(last_time)) = (&state.last_click_node_id, state.last_click_time) {
                            last_id == &node.id && now.duration_since(last_time).as_millis() < 350
                        } else {
                            false
                        };

                        state.last_click_time = Some(now);
                        state.last_click_node_id = Some(node.id.clone());

                        if is_double_click {
                            if let FlowNodeType::Scene { scene_name } = &node.node_type {
                                state.cache.clear();
                                return (
                                    canvas::event::Status::Captured,
                                    Some(FlowCanvasAction::OpenSceneInEditor(scene_name.clone())),
                                );
                            }
                        }

                        // Start dragging
                        state.dragging_node_id = Some(node.id.clone());
                        state.drag_offset_x = (cursor_position.x - pan_x) / zoom - node.x;
                        state.drag_offset_y = (cursor_position.y - pan_y) / zoom - node.y;
                        state.cache.clear();
                        return (
                            canvas::event::Status::Captured,
                            Some(FlowCanvasAction::SelectNode(node.id.clone())),
                        );
                    }
                }

                // Clicked on empty canvas
                state.connecting_pin = None;
                state.cache.clear();
                (canvas::event::Status::Captured, Some(FlowCanvasAction::ClearSelection))
            }

            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right | mouse::Button::Middle)) => {
                state.is_panning = true;
                state.pan_start = Some(cursor_position);
                state.cache.clear();
                (canvas::event::Status::Captured, None)
            }

            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                let mut action = None;
                if let Some(pos) = cursor.position_in(bounds) {
                    if let Some(node_id) = &state.dragging_node_id {
                        let zoom = self.graph.zoom;
                        let pan_x = self.graph.pan_x;
                        let pan_y = self.graph.pan_y;
                        let raw_x = (pos.x - pan_x) / zoom - state.drag_offset_x;
                        let raw_y = (pos.y - pan_y) / zoom - state.drag_offset_y;
                        let (new_x, new_y) = if self.snap_grid {
                            ((raw_x / 20.0).round() * 20.0, (raw_y / 20.0).round() * 20.0)
                        } else {
                            (raw_x, raw_y)
                        };
                        action = Some(FlowCanvasAction::SetNodePosition {
                            node_id: node_id.clone(),
                            x: new_x,
                            y: new_y,
                        });
                        state.cache.clear();
                    } else if state.is_panning {
                        if let Some(start) = state.pan_start {
                            let dx = pos.x - start.x;
                            let dy = pos.y - start.y;
                            state.pan_start = Some(pos);
                            action = Some(FlowCanvasAction::PanCanvas { dx, dy });
                            state.cache.clear();
                        }
                    } else if state.connecting_pin.is_some() {
                        state.cache.clear();
                    }
                }
                let _ = position;
                (canvas::event::Status::Captured, action)
            }

            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                let mut action = None;
                if let Some((from_node, from_pin)) = state.connecting_pin.take() {
                    let zoom = self.graph.zoom;
                    let pan_x = self.graph.pan_x;
                    let pan_y = self.graph.pan_y;

                    for node in &self.graph.nodes {
                        if self.node_has_input_pin(node) && node.id != from_node {
                            let pin_x = node.x * zoom + pan_x;
                            let pin_y = (node.y + 85.0) * zoom + pan_y;
                            let dist = ((cursor_position.x - pin_x).powi(2) + (cursor_position.y - pin_y).powi(2)).sqrt();
                            if dist <= 22.0 * zoom {
                                action = Some(FlowCanvasAction::ConnectPins {
                                    from_node: from_node.clone(),
                                    from_pin: from_pin.clone(),
                                    to_node: node.id.clone(),
                                    to_pin: "in".to_string(),
                                });
                                break;
                            }
                        }
                    }
                }
                state.dragging_node_id = None;
                state.cache.clear();
                (canvas::event::Status::Captured, action)
            }

            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Right | mouse::Button::Middle)) => {
                state.is_panning = false;
                state.pan_start = None;
                state.cache.clear();
                (canvas::event::Status::Captured, None)
            }

            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                let factor = match delta {
                    mouse::ScrollDelta::Lines { y, .. } => y * 0.1,
                    mouse::ScrollDelta::Pixels { y, .. } => y * 0.005,
                };
                state.cache.clear();
                (canvas::event::Status::Captured, Some(FlowCanvasAction::ZoomDelta(factor)))
            }

            _ => (canvas::event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geom = state.cache.draw(renderer, bounds.size(), |frame| {
            let zoom = self.graph.zoom;
            let pan_x = self.graph.pan_x;
            let pan_y = self.graph.pan_y;

            // 1. Background Canvas
            frame.fill_rectangle(
                Point::ORIGIN,
                bounds.size(),
                Color::from_rgb8(15, 23, 42),
            );

            // 2. Dot Grid
            let grid_step = (36.0 * zoom).max(18.0);
            let start_x = (pan_x % grid_step) - grid_step;
            let start_y = (pan_y % grid_step) - grid_step;
            let mut gx = start_x;
            while gx < bounds.width + grid_step {
                let mut gy = start_y;
                while gy < bounds.height + grid_step {
                    frame.fill_rectangle(
                        Point::new(gx, gy),
                        Size::new(2.0, 2.0),
                        Color::from_rgba8(71, 85, 105, 0.4),
                    );
                    gy += grid_step;
                }
                gx += grid_step;
            }

            // 3. Draw Connections (Bézier Curves)
            for conn in &self.graph.connections {
                if let (Some(from_pos), Some(to_pos)) = (
                    self.get_pin_screen_pos(&conn.from_node_id, &conn.from_pin_id, true),
                    self.get_pin_screen_pos(&conn.to_node_id, &conn.to_pin_id, false),
                ) {
                    let (cp1, cp2) = if to_pos.x >= from_pos.x {
                        let dist_x = to_pos.x - from_pos.x;
                        let dx = (dist_x * 0.45).clamp(12.0 * zoom, 180.0 * zoom).min(dist_x * 0.48);
                        (
                            Point::new(from_pos.x + dx, from_pos.y),
                            Point::new(to_pos.x - dx, to_pos.y),
                        )
                    } else {
                        // Backward loop (e.g. Return to Title back to Main Menu)
                        let loop_x = 60.0 * zoom;
                        let dip_y = ((from_pos.x - to_pos.x).abs() * 0.12).clamp(70.0 * zoom, 140.0 * zoom);
                        (
                            Point::new(from_pos.x + loop_x, from_pos.y + dip_y),
                            Point::new(to_pos.x - loop_x, to_pos.y + dip_y),
                        )
                    };

                    let path = Path::new(|builder| {
                        builder.move_to(from_pos);
                        builder.bezier_curve_to(cp1, cp2, to_pos);
                    });

                    // Crisp, sharp high-contrast wire
                    frame.stroke(
                        &path,
                        Stroke::default()
                            .with_color(Color::from_rgb8(140, 155, 255))
                            .with_width(3.0 * zoom),
                    );

                    // Destination circle dot
                    let dest_dot = Path::circle(to_pos, 4.0 * zoom);
                    frame.fill(&dest_dot, Color::from_rgb8(195, 205, 255));
                }
            }

            // 4. In-Flight Wire (interactive drag)
            if let Some((from_node, from_pin)) = &state.connecting_pin {
                if let (Some(from_pos), Some(mouse_pos)) = (
                    self.get_pin_screen_pos(from_node, from_pin, true),
                    state.current_mouse_pos,
                ) {
                    let (cp1, cp2) = if mouse_pos.x >= from_pos.x {
                        let dist_x = mouse_pos.x - from_pos.x;
                        let dx = (dist_x * 0.45).clamp(12.0 * zoom, 180.0 * zoom).min(dist_x * 0.48);
                        (
                            Point::new(from_pos.x + dx, from_pos.y),
                            Point::new(mouse_pos.x - dx, mouse_pos.y),
                        )
                    } else {
                        let loop_x = 60.0 * zoom;
                        let dip_y = ((from_pos.x - mouse_pos.x).abs() * 0.12).clamp(70.0 * zoom, 140.0 * zoom);
                        (
                            Point::new(from_pos.x + loop_x, from_pos.y + dip_y),
                            Point::new(mouse_pos.x - loop_x, mouse_pos.y + dip_y),
                        )
                    };

                    let path = Path::new(|builder| {
                        builder.move_to(from_pos);
                        builder.bezier_curve_to(cp1, cp2, mouse_pos);
                    });

                    frame.stroke(
                        &path,
                        Stroke::default()
                            .with_color(Color::from_rgb8(245, 158, 11))
                            .with_width(3.0 * zoom),
                    );
                }
            }

            // 5. Draw Nodes
            for node in &self.graph.nodes {
                let nx = node.x * zoom + pan_x;
                let ny = node.y * zoom + pan_y;
                let nw = self.get_node_width(node) * zoom;
                let nh = self.get_node_height(node) * zoom;
                let is_selected = self.selected_node_id == Some(node.id.as_str());

                // Node Body Background
                frame.fill_rectangle(
                    Point::new(nx, ny),
                    Size::new(nw, nh),
                    Color::from_rgb8(24, 27, 36),
                );

                // Node Header Background
                let header_h = 34.0 * zoom;
                let header_color = match &node.node_type {
                    FlowNodeType::MainMenu => Color::from_rgb8(16, 185, 129), // Emerald
                    FlowNodeType::PauseMenu => Color::from_rgb8(14, 165, 233), // Sky Blue
                    FlowNodeType::Settings => Color::from_rgb8(99, 102, 241), // Indigo
                    FlowNodeType::SaveLoad => Color::from_rgb8(20, 184, 166), // Teal
                    FlowNodeType::Ending { .. } => Color::from_rgb8(244, 63, 94), // Rose
                    FlowNodeType::Scene { scene_name } => {
                        if self.scene_has_choice(scene_name) {
                            Color::from_rgb8(245, 158, 11) // Amber
                        } else {
                            Color::from_rgb8(99, 102, 241) // Indigo
                        }
                    }
                };

                frame.fill_rectangle(
                    Point::new(nx, ny),
                    Size::new(nw, header_h),
                    header_color,
                );

                // Node Border
                let border_color = if is_selected {
                    Color::from_rgb8(165, 180, 252)
                } else {
                    Color::from_rgb8(51, 65, 85)
                };
                let border_width = if is_selected { 2.5 * zoom } else { 1.0 * zoom };

                let border_path = Path::rectangle(Point::new(nx, ny), Size::new(nw, nh));
                frame.stroke(
                    &border_path,
                    Stroke::default()
                        .with_color(border_color)
                        .with_width(border_width),
                );

                // Header Title Text (clipped so it never overflows the node header on zoom out)
                if zoom >= 0.25 {
                    let title_clip_w = if node.id != "system_main_menu" {
                        (nw - 34.0 * zoom).max(0.0)
                    } else {
                        (nw - 16.0 * zoom).max(0.0)
                    };
                    frame.with_clip(
                        Rectangle {
                            x: nx,
                            y: ny,
                            width: title_clip_w,
                            height: header_h,
                        },
                        |frame| {
                            frame.fill_text(Text {
                                content: node.title.clone(),
                                position: Point::new(12.0 * zoom, 9.0 * zoom),
                                color: Color::WHITE,
                                size: (13.0 * zoom).max(1.0).into(),
                                ..Default::default()
                            });
                        },
                    );
                }

                // Vector Delete Button [X] in header (Immune to font glyph issues)
                if node.id != "system_main_menu" {
                    let cx = nx + nw - 18.0 * zoom;
                    let cy = ny + 17.0 * zoom;
                    let arm = 4.5 * zoom;
                    let x_path = Path::new(|b| {
                        b.move_to(Point::new(cx - arm, cy - arm));
                        b.line_to(Point::new(cx + arm, cy + arm));
                        b.move_to(Point::new(cx + arm, cy - arm));
                        b.line_to(Point::new(cx - arm, cy + arm));
                    });
                    frame.stroke(
                        &x_path,
                        Stroke::default()
                            .with_color(Color::from_rgb8(254, 202, 202))
                            .with_width(1.8 * zoom),
                    );
                }

                // Subtitle / Info Tag (Dedicated area between header and divider)
                let info_text = match &node.node_type {
                    FlowNodeType::MainMenu => "Entry Screen (Starts New Game)".to_string(),
                    FlowNodeType::PauseMenu => "System: In-Game Pause (ESC Menu)".to_string(),
                    FlowNodeType::Settings => "System: Audio, Video & Text Prefs".to_string(),
                    FlowNodeType::SaveLoad => "System: Progress Save & Load Slots".to_string(),
                    FlowNodeType::Ending { ending_name } => format!("Arc End: {}", ending_name),
                    FlowNodeType::Scene { scene_name } => {
                        let block_count = self.get_scene_block_count(scene_name);
                        if self.scene_has_choice(scene_name) {
                            format!("{} blocks · Branching Choices", block_count)
                        } else {
                            format!("{} blocks · Sequential Flow", block_count)
                        }
                    }
                };

                let pill_rect = Path::rectangle(
                    Point::new(nx + 10.0 * zoom, ny + 42.0 * zoom),
                    Size::new(nw - 20.0 * zoom, 22.0 * zoom),
                );
                frame.fill(&pill_rect, Color::from_rgba8(30, 41, 59, 0.7));

                if zoom >= 0.42 {
                    frame.with_clip(
                        Rectangle {
                            x: nx + 12.0 * zoom,
                            y: ny + 42.0 * zoom,
                            width: (nw - 24.0 * zoom).max(0.0),
                            height: 22.0 * zoom,
                        },
                        |frame| {
                            frame.fill_text(Text {
                                content: info_text,
                                position: Point::new(4.0 * zoom, 5.0 * zoom),
                                color: Color::from_rgb8(148, 163, 184),
                                size: (11.0 * zoom).max(1.0).into(),
                                ..Default::default()
                            });
                        },
                    );
                }

                // Subtle Divider line between info pill and pins
                let div_path = Path::new(|b| {
                    b.move_to(Point::new(nx + 10.0 * zoom, ny + 72.0 * zoom));
                    b.line_to(Point::new(nx + nw - 10.0 * zoom, ny + 72.0 * zoom));
                });
                frame.stroke(
                    &div_path,
                    Stroke::default()
                        .with_color(Color::from_rgba8(51, 65, 85, 0.5))
                        .with_width(1.0 * zoom),
                );

                // --- Input Pin (Left Edge) ---
                let pin_start_y = ny + 85.0 * zoom;
                if self.node_has_input_pin(node) {
                    let pin_x = nx;
                    let pin_y = pin_start_y;
                    let circle = Path::circle(Point::new(pin_x, pin_y), 6.0 * zoom);
                    frame.fill(&circle, Color::from_rgb8(52, 211, 153)); // Emerald
                    frame.stroke(
                        &circle,
                        Stroke::default()
                            .with_color(Color::from_rgb8(15, 23, 42))
                            .with_width(2.0 * zoom),
                    );

                    if zoom >= 0.42 {
                        let in_label = if node.id == "system_main_menu" {
                            "In / Loop"
                        } else {
                            "In"
                        };

                        frame.with_clip(
                            Rectangle {
                                x: nx + 8.0 * zoom,
                                y: ny + header_h,
                                width: (nw - 16.0 * zoom).max(0.0),
                                height: (nh - header_h).max(0.0),
                            },
                            |frame| {
                                frame.fill_text(Text {
                                    content: in_label.to_string(),
                                    position: Point::new(6.0 * zoom, (pin_y - ny - header_h) - 7.0 * zoom),
                                    color: Color::from_rgb8(148, 163, 184),
                                    size: (11.0 * zoom).max(1.0).into(),
                                    ..Default::default()
                                });
                            },
                        );
                    }
                }

                // --- Output Pins (Right Edge) ---
                let output_pins = self.get_node_output_pins(node);
                for (p_idx, (_pin_id, label)) in output_pins.iter().enumerate() {
                    let pin_x = nx + nw;
                    let pin_y = pin_start_y + (p_idx as f32 * 30.0) * zoom;
                    let pin_color = if self.scene_has_choice(node.scene_name().unwrap_or("")) {
                        Color::from_rgb8(245, 158, 11) // Amber for choices
                    } else {
                        Color::from_rgb8(129, 140, 248) // Indigo
                    };

                    let circle = Path::circle(Point::new(pin_x, pin_y), 6.0 * zoom);
                    frame.fill(&circle, pin_color);
                    frame.stroke(
                        &circle,
                        Stroke::default()
                            .with_color(Color::from_rgb8(15, 23, 42))
                            .with_width(2.0 * zoom),
                    );

                    if zoom >= 0.42 {
                        // Pin Label inside node (right-aligned & clipped within node bounds)
                        let clip_w = (nw - 60.0 * zoom).max(0.0);
                        frame.with_clip(
                            Rectangle {
                                x: nx + 48.0 * zoom,
                                y: pin_y - 11.0 * zoom,
                                width: clip_w,
                                height: 22.0 * zoom,
                            },
                            |frame| {
                                frame.fill_text(Text {
                                    content: label.clone(),
                                    position: Point::new((clip_w - 2.0 * zoom).max(0.0), 4.0 * zoom),
                                    color: Color::from_rgb8(226, 232, 240),
                                    size: (11.0 * zoom).max(1.0).into(),
                                    horizontal_alignment: iced::alignment::Horizontal::Right,
                                    ..Default::default()
                                });
                            },
                        );
                    }
                }

                // Warning indicator if scene node is unconnected
                if zoom >= 0.42 {
                    if let FlowNodeType::Scene { .. } = &node.node_type {
                        let is_connected = self.graph.connections.iter().any(|c| c.from_node_id == node.id);
                        if !is_connected {
                            frame.with_clip(
                                Rectangle {
                                    x: nx + 8.0 * zoom,
                                    y: ny + header_h,
                                    width: (nw - 16.0 * zoom).max(0.0),
                                    height: (nh - header_h).max(0.0),
                                },
                                |frame| {
                                    frame.fill_text(Text {
                                        content: "! Unconnected".to_string(),
                                        position: Point::new(6.0 * zoom, (nh - header_h) - 14.0 * zoom),
                                        color: Color::from_rgb8(251, 191, 36),
                                        size: (10.0 * zoom).max(1.0).into(),
                                        ..Default::default()
                                    });
                                },
                            );
                        }
                    }
                }
            }
        });

        vec![geom]
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> mouse::Interaction {
        if state.is_panning {
            return mouse::Interaction::Grabbing;
        }
        if state.dragging_node_id.is_some() {
            return mouse::Interaction::Grabbing;
        }
        if self.mode == FlowInteractionMode::CutWire {
            return mouse::Interaction::Crosshair;
        }
        if self.mode == FlowInteractionMode::Connect {
            return mouse::Interaction::Crosshair;
        }
        if let Some(pos) = cursor.position_in(bounds) {
            let zoom = self.graph.zoom;
            let pan_x = self.graph.pan_x;
            let pan_y = self.graph.pan_y;

            // Check if hovering a pin
            for node in &self.graph.nodes {
                let nx = node.x * zoom + pan_x;
                let ny = node.y * zoom + pan_y;
                let nw = self.get_node_width(node) * zoom;
                let pin_start_y = ny + 85.0 * zoom;

                let output_pins = self.get_node_output_pins(node);
                for (p_idx, _) in output_pins.iter().enumerate() {
                    let pin_x = nx + nw;
                    let pin_y = pin_start_y + (p_idx as f32 * 30.0) * zoom;
                    let dist = ((pos.x - pin_x).powi(2) + (pos.y - pin_y).powi(2)).sqrt();
                    if dist <= 14.0 * zoom {
                        return mouse::Interaction::Crosshair;
                    }
                }

                if self.node_has_input_pin(node) {
                    let pin_x = nx;
                    let pin_y = pin_start_y;
                    let dist = ((pos.x - pin_x).powi(2) + (pos.y - pin_y).powi(2)).sqrt();
                    if dist <= 14.0 * zoom {
                        return mouse::Interaction::Crosshair;
                    }
                }
            }
        }
        mouse::Interaction::default()
    }
}

impl<'a> FlowCanvasWidget<'a> {
    fn node_has_input_pin(&self, _node: &FlowNode) -> bool {
        true
    }

    fn get_node_width(&self, node: &FlowNode) -> f32 {
        let max_label_len = self.get_node_output_pins(node).iter().map(|(_, l)| l.len()).max().unwrap_or(10);
        (max_label_len as f32 * 8.0 + 80.0).max(280.0).min(440.0)
    }

    fn get_node_height(&self, node: &FlowNode) -> f32 {
        let pin_count = self.get_node_output_pins(node).len().max(1);
        82.0 + pin_count as f32 * 30.0
    }

    fn get_node_output_pins(&self, node: &FlowNode) -> Vec<(String, String)> {
        match &node.node_type {
            FlowNodeType::MainMenu => {
                vec![("new_game".to_string(), "New Game ->".to_string())]
            }
            FlowNodeType::PauseMenu => {
                vec![
                    ("resume".to_string(), "Resume Story ->".to_string()),
                    ("settings".to_string(), "Settings ->".to_string()),
                    ("title".to_string(), "Return to Title ->".to_string()),
                ]
            }
            FlowNodeType::Settings => {
                vec![("back".to_string(), "Back / Return ->".to_string())]
            }
            FlowNodeType::SaveLoad => {
                vec![("back".to_string(), "Back to Story ->".to_string())]
            }
            FlowNodeType::Ending { .. } => {
                vec![("title".to_string(), "To Title Screen ->".to_string())]
            }
            FlowNodeType::Scene { scene_name } => {
                if let Some(scene) = self.scenes.iter().find(|s| &s.name == scene_name) {
                    // Check if scene has Choice blocks
                    for block in scene.blocks.iter().rev() {
                        if let ActionBlock::Choice { options, .. } = block {
                            if !options.is_empty() {
                                return options
                                    .iter()
                                    .enumerate()
                                    .map(|(idx, opt)| {
                                        (format!("choice_{}", idx), format!("{} ->", opt.text))
                                    })
                                    .collect();
                            }
                        }
                    }
                }
                vec![("next".to_string(), "Next Scene ->".to_string())]
            }
        }
    }

    fn get_pin_screen_pos(&self, node_id: &str, pin_id: &str, is_output: bool) -> Option<Point> {
        let node = self.graph.find_node(node_id)?;
        let zoom = self.graph.zoom;
        let pan_x = self.graph.pan_x;
        let pan_y = self.graph.pan_y;
        let nx = node.x * zoom + pan_x;
        let ny = node.y * zoom + pan_y;
        let nw = self.get_node_width(node) * zoom;
        let pin_start_y = ny + 85.0 * zoom;

        if is_output {
            let pins = self.get_node_output_pins(node);
            let idx = pins.iter().position(|(p, _)| p == pin_id)?;
            Some(Point::new(nx + nw, pin_start_y + (idx as f32 * 30.0) * zoom))
        } else {
            Some(Point::new(nx, pin_start_y))
        }
    }

    fn scene_has_choice(&self, scene_name: &str) -> bool {
        self.scenes
            .iter()
            .find(|s| &s.name == scene_name)
            .map(|s| s.blocks.iter().any(|b| matches!(b, ActionBlock::Choice { .. })))
            .unwrap_or(false)
    }

    fn get_scene_block_count(&self, scene_name: &str) -> usize {
        self.scenes
            .iter()
            .find(|s| &s.name == scene_name)
            .map(|s| s.blocks.len())
            .unwrap_or(0)
    }
}
