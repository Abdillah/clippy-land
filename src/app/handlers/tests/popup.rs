use super::*;

#[test]
fn popup_closed_clears_popup_and_search() {
    let mut app = AppModel::default();
    let id = cosmic::iced::window::Id::unique();
    app.popup = Some(id);
    app.popup_is_layer_surface = true;
    app.search_query = "hello".into();
    app.hovered_index = Some(1);
    app.at_scroll_bottom = true;

    dispatch(&mut app, Message::PopupClosed(id));

    assert!(app.popup.is_none());
    assert!(!app.popup_is_layer_surface);
    assert!(app.search_query.is_empty());
    assert!(app.hovered_index.is_none());
    assert!(!app.at_scroll_bottom);
}

#[test]
fn popup_closed_ignores_mismatched_id() {
    let mut app = AppModel::default();
    let real_id = cosmic::iced::window::Id::unique();
    let other_id = cosmic::iced::window::Id::unique();
    app.popup = Some(real_id);
    app.search_query = "query".into();

    dispatch(&mut app, Message::PopupClosed(other_id));

    assert_eq!(app.popup, Some(real_id));
    assert_eq!(app.search_query, "query");
}

#[test]
fn popup_open_trace_starts_on_open_and_clears_after_first_view() {
    let mut app = AppModel::default();
    app.history.push_back(text_item("row", false));
    app.recompute_filtered_indices();
    let popup_id = cosmic::iced::window::Id::unique();
    app.popup = Some(popup_id);

    app.begin_popup_open_trace("test");
    assert!(app.popup_open_trace_pending_for_test());

    let _ = view::view_window(&app, popup_id);
    assert!(app.popup_open_trace_pending_for_test());

    dispatch(&mut app, Message::PopupRedraw(popup_id));
    assert!(!app.popup_open_trace_pending_for_test());
}

#[test]
fn popup_closed_cancels_pending_popup_open_trace() {
    let mut app = AppModel::default();
    app.history.push_back(text_item("row", false));
    app.recompute_filtered_indices();
    let popup_id = cosmic::iced::window::Id::unique();
    app.popup = Some(popup_id);

    app.begin_popup_open_trace("test");
    assert!(app.popup_open_trace_pending_for_test());

    dispatch(&mut app, Message::PopupClosed(popup_id));
    assert!(!app.popup_open_trace_pending_for_test());
}

#[test]
fn prewarm_for_first_popup_caches_existing_image_thumbnails() {
    let mut app = AppModel::default();
    app.history.push_back(HistoryItem {
        entry: image_entry(17),
        pinned: false,
    });

    prewarm_for_first_popup(&mut app);

    assert_eq!(app.thumbnail_handles.len(), 1);
}

#[test]
fn window_unfocused_only_closes_layer_surface_popups() {
    let mut app = AppModel::default();
    let id = cosmic::iced::window::Id::unique();
    app.popup = Some(id);
    app.popup_is_layer_surface = false;
    app.search_query = "query".into();

    dispatch(&mut app, Message::WindowUnfocused(id));

    assert!(app.popup.is_some());
    assert_eq!(app.search_query, "query");
}
