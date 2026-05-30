use super::shared::warm_thumbnail_handles;
use crate::app::{AppModel, Message};
use cosmic::iced::Limits;
use cosmic::iced::platform_specific::shell::wayland::commands::layer_surface::{
    self, KeyboardInteractivity, destroy_layer_surface, get_layer_surface,
};
use cosmic::iced::platform_specific::shell::wayland::commands::popup::{destroy_popup, get_popup};
use cosmic::iced::runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings;
use cosmic::prelude::*;
use std::time::Instant;

pub(super) fn handle(
    app: &mut AppModel,
    message: Message,
) -> Option<Task<cosmic::Action<Message>>> {
    match message {
        Message::TogglePopup => Some(toggle_popup(app)),
        Message::ToggleViaIpc => Some(toggle_via_ipc(app)),
        Message::PopupOpened(id) => {
            if app.popup.as_ref() == Some(&id) {
                app.note_popup_opened();
            }
            Some(Task::none())
        }
        Message::PopupRedraw(id) => {
            if app.popup.as_ref() == Some(&id) {
                app.finish_popup_open_trace_on_redraw();
            }
            Some(Task::none())
        }
        Message::WindowUnfocused(id) => Some(window_unfocused(app, id)),
        Message::PopupClosed(id) => {
            if app.popup.as_ref() == Some(&id) {
                app.popup = None;
                app.popup_is_layer_surface = false;
                app.search_query.clear();
                app.settings_open = false;
                app.settings_error = None;
                app.hovered_index = None;
                app.at_scroll_bottom = false;
                app.history_viewport = None;
                app.text_overlay_index = None;
                app.cancel_popup_open_trace("popup closed before first redraw");
            }
            Some(Task::none())
        }
        _ => None,
    }
}

pub(super) fn warm_for_first_popup(app: &mut AppModel) {
    warm_thumbnail_handles(app);
}

fn toggle_popup(app: &mut AppModel) -> Task<cosmic::Action<Message>> {
    if let Some(p) = app.popup.take() {
        let is_layer = app.popup_is_layer_surface;
        app.popup_is_layer_surface = false;
        app.search_query.clear();
        app.settings_open = false;
        app.settings_error = None;
        app.text_overlay_index = None;
        app.cancel_popup_open_trace("popup toggled closed before first view");
        if is_layer {
            destroy_layer_surface(p)
        } else {
            destroy_popup(p)
        }
    } else {
        app.begin_popup_open_trace("icon-click");
        let new_id = cosmic::iced::window::Id::unique();
        app.popup.replace(new_id);
        app.popup_is_layer_surface = false;
        let popup_settings = app.core.applet.get_popup_settings(
            app.core.main_window_id().unwrap(),
            new_id,
            None,
            None,
            None,
        );
        app.note_popup_stage_marker("issuing get_popup request");
        get_popup(popup_settings)
    }
}

fn toggle_via_ipc(app: &mut AppModel) -> Task<cosmic::Action<Message>> {
    if let Some(p) = app.popup.take() {
        let is_layer = app.popup_is_layer_surface;
        app.popup_is_layer_surface = false;
        app.search_query.clear();
        app.settings_open = false;
        app.settings_error = None;
        app.text_overlay_index = None;
        app.cancel_popup_open_trace("ipc toggle closed popup before first view");
        if is_layer {
            destroy_layer_surface(p)
        } else {
            destroy_popup(p)
        }
    } else {
        app.begin_popup_open_trace("ipc-toggle");
        let warm_started = Instant::now();
        warm_thumbnail_handles(app);
        app.note_popup_stage_duration("warm_thumbnail_handles complete", warm_started.elapsed());
        let new_id = cosmic::iced::window::Id::unique();
        app.popup.replace(new_id);
        app.popup_is_layer_surface = true;
        app.note_popup_stage_marker("issuing get_layer_surface request");
        get_layer_surface(SctkLayerSurfaceSettings {
            id: new_id,
            keyboard_interactivity: KeyboardInteractivity::OnDemand,
            anchor: layer_surface::Anchor::TOP
                | layer_surface::Anchor::LEFT
                | layer_surface::Anchor::RIGHT,
            namespace: "clippy-land".into(),
            size: Some((None, Some(400))),
            size_limits: Limits::NONE.min_width(1.0).min_height(1.0),
            ..Default::default()
        })
    }
}

fn window_unfocused(
    app: &mut AppModel,
    id: cosmic::iced::window::Id,
) -> Task<cosmic::Action<Message>> {
    if app.popup.as_ref() == Some(&id) && app.popup_is_layer_surface {
        if let Some(p) = app.popup.take() {
            app.popup_is_layer_surface = false;
            app.search_query.clear();
            app.settings_open = false;
            app.settings_error = None;
            app.hovered_index = None;
            app.at_scroll_bottom = false;
            app.history_viewport = None;
            app.text_overlay_index = None;
            app.cancel_popup_open_trace("window lost focus before first redraw");
            destroy_layer_surface(p)
        } else {
            Task::none()
        }
    } else {
        Task::none()
    }
}
