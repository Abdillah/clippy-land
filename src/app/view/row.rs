use super::style::{
    highlight_history_target, pressed_entry_button_style, transparent_entry_button_style,
    transparent_icon_button_style,
};
use super::summary::{summarize_one_line, summarize_one_line_with_limit};
use crate::app::model::{FocusPart, HistoryItem};
use crate::app::{AppModel, Message, icons};
use crate::fl;
use crate::services::clipboard;
use cosmic::iced::widget::image::Handle as ImageHandle;
use cosmic::iced::{Alignment, Length};
use cosmic::prelude::*;
use cosmic::widget;

pub(super) fn history_row<'a>(
    app: &'a AppModel,
    idx: usize,
    item: &'a HistoryItem,
) -> Element<'a, Message> {
    const TEXT_EXPANDED_MAX_CHARS: usize = 150;
    const IMAGE_PREVIEW_COLLAPSED_HEIGHT: f32 = 120.0;
    const IMAGE_PREVIEW_EXPANDED_HEIGHT: f32 = 240.0;

    let row_is_active = app
        .hovered_focus
        .is_some_and(|(focus_idx, _)| focus_idx == idx)
        || app
            .keyboard_focus
            .is_some_and(|(focus_idx, _)| focus_idx == idx);
    let row_alignment = if matches!(&item.entry, clipboard::ClipboardEntry::Image { .. }) {
        Alignment::Start
    } else {
        Alignment::Center
    };

    let label: Element<'_, Message> = match &item.entry {
        clipboard::ClipboardEntry::Text(text) => {
            let summary = if row_is_active {
                summarize_one_line_with_limit(text, TEXT_EXPANDED_MAX_CHARS)
            } else {
                summarize_one_line(text)
            };
            widget::text::body(summary).into()
        }
        clipboard::ClipboardEntry::Image {
            mime,
            bytes,
            thumbnail_png,
            ..
        } => {
            let thumb: Option<Element<'_, Message>> = thumbnail_png.as_ref().map(|png| {
                let preview_height = if row_is_active {
                    IMAGE_PREVIEW_EXPANDED_HEIGHT
                } else {
                    IMAGE_PREVIEW_COLLAPSED_HEIGHT
                };

                widget::container(
                    widget::image::<ImageHandle>(ImageHandle::from_bytes(png.clone()))
                        .width(Length::Fill)
                        .height(Length::Fixed(preview_height))
                        .content_fit(cosmic::iced::ContentFit::Contain)
                        .expand(false),
                )
                .width(Length::Fill)
                .height(Length::Fixed(preview_height))
                .max_height(preview_height)
                .clip(true)
                .into()
            });

            let mut col = widget::column::Column::new()
                .width(Length::Fill)
                .align_x(Alignment::Center);
            if let Some(thumb) = thumb {
                col = col.push(thumb);
            }
            let details = if row_is_active {
                format!(
                    "{} ({} KB)",
                    mime,
                    (bytes.len().saturating_add(1023)) / 1024
                )
            } else {
                "\u{00A0}".to_string()
            };

            col = col.push(widget::text::caption(details).width(Length::Fill));
            col.into()
        }
    };

    let copy_button = widget::button::custom(label)
        .class(cosmic::theme::Button::Custom {
            active: Box::new(|_, theme| transparent_entry_button_style(theme)),
            disabled: Box::new(transparent_entry_button_style),
            hovered: Box::new(|_, theme| transparent_entry_button_style(theme)),
            pressed: Box::new(|_, theme| pressed_entry_button_style(theme)),
        })
        .on_press(Message::CopyFromHistory(idx))
        .width(Length::Fill)
        .padding([8, 12]);

    let entry_active = app.keyboard_focus == Some((idx, FocusPart::Entry))
        || app.hovered_focus == Some((idx, FocusPart::Entry));

    let copy_button_elem = widget::mouse_area(highlight_history_target(
        widget::container(copy_button).width(Length::Fill).into(),
        entry_active,
    ))
    .on_enter(Message::HoverEntry(Some((idx, FocusPart::Entry))))
    .on_exit(Message::HoverEntry(None));

    let pin_button = widget::button::icon(if item.pinned {
        icons::pin_icon_pinned()
    } else {
        icons::pin_icon()
    })
    .class(cosmic::theme::Button::Custom {
        active: Box::new(|_, theme| transparent_icon_button_style(theme)),
        disabled: Box::new(transparent_icon_button_style),
        hovered: Box::new(|_, theme| transparent_icon_button_style(theme)),
        pressed: Box::new(|_, theme| transparent_icon_button_style(theme)),
    })
    .tooltip(if item.pinned {
        fl!("unpin")
    } else {
        fl!("pin")
    })
    .on_press(Message::TogglePin(idx))
    .extra_small()
    .width(Length::Shrink);

    let remove_button = widget::button::icon(icons::remove_icon())
        .class(cosmic::theme::Button::Custom {
            active: Box::new(|_, theme| transparent_icon_button_style(theme)),
            disabled: Box::new(transparent_icon_button_style),
            hovered: Box::new(|_, theme| transparent_icon_button_style(theme)),
            pressed: Box::new(|_, theme| transparent_icon_button_style(theme)),
        })
        .tooltip(fl!("remove"))
        .on_press(Message::RemoveHistory(idx))
        .extra_small()
        .width(Length::Shrink);

    let pin_active = app.keyboard_focus == Some((idx, FocusPart::Pin))
        || app.hovered_focus == Some((idx, FocusPart::Pin));
    let remove_active = app.keyboard_focus == Some((idx, FocusPart::Remove))
        || app.hovered_focus == Some((idx, FocusPart::Remove));

    let pin_button_elem =
        widget::mouse_area(highlight_history_target(pin_button.into(), pin_active))
            .on_enter(Message::HoverEntry(Some((idx, FocusPart::Pin))))
            .on_exit(Message::HoverEntry(None));

    let remove_button_elem = widget::mouse_area(highlight_history_target(
        remove_button.into(),
        remove_active,
    ))
    .on_enter(Message::HoverEntry(Some((idx, FocusPart::Remove))))
    .on_exit(Message::HoverEntry(None));

    let actions = widget::column::Column::new()
        .spacing(2)
        .align_x(Alignment::Center)
        .push(pin_button_elem)
        .push(remove_button_elem);

    let entry = widget::row::Row::new()
        .push(copy_button_elem)
        .push(
            widget::container(actions)
                .width(Length::Fixed(40.0))
                .padding([0, 2]),
        )
        .align_y(row_alignment)
        .width(Length::Fill);

    widget::container(entry)
        .class(cosmic::theme::Container::Card)
        .width(Length::Fill)
        .clip(true)
        .into()
}
