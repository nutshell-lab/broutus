use super::StartupCollection;
use crate::game::GameState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::EguiContext;

pub fn show_main_menu(
    mut egui_context: ResMut<EguiContext>,
    mut game_state: ResMut<State<GameState>>,
    mut ew_exit: EventWriter<AppExit>,
    collection: Res<StartupCollection>,
    windows: Res<Windows>,
) {
    egui_context.set_egui_texture(0, collection.splash.clone());
    egui_context.set_egui_texture(1, collection.start.clone());
    egui_context.set_egui_texture(2, collection.options.clone());
    egui_context.set_egui_texture(3, collection.exit.clone());

    let window = windows.get_primary().unwrap();
    egui::Window::new("broutus")
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .fixed_rect(egui::Rect::from_two_pos(
            (0., 0.).into(),
            (window.width(), window.height()).into(),
        ))
        .frame(
            egui::Frame::default()
                .stroke(egui::Stroke::none())
                .fill(egui::Color32::from_black_alpha(0)),
        )
        .show(egui_context.ctx_mut(), |ui| {
            ui.centered_and_justified(|ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.image(egui::TextureId::User(0), (768., 480.));
                    let start = ui.add(egui::ImageButton::new(
                        egui::TextureId::User(1),
                        (152., 47.),
                    ));
                    let options = ui.add(egui::ImageButton::new(
                        egui::TextureId::User(2),
                        (203., 52.),
                    ));
                    let exit = ui.add(egui::ImageButton::new(
                        egui::TextureId::User(3),
                        (119., 54.),
                    ));

                    if start.clicked() {
                        game_state.set(GameState::Arena).unwrap();
                    }

                    if exit.clicked() {
                        ew_exit.send(AppExit);
                    }
                });
            });
        });
}