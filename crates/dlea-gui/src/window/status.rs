//! The window's footer joins the pane count and disk facts around fb1's message service.
use crate::{
    g2::status_feedback::{
        line::{ArmLost, MessageLine, NoticeEnded},
        observe::Paint,
    },
    list::{Metrics, Palette, Typeface},
    pane::states::Kind,
    state::Shell,
    strips::status_host,
};
use dlea::{
    i18n::I18n,
    ui_logic_status_feedback::{messages::Messages, status::Hint},
};
use gpui::{div, prelude::*, px, AppContext, Context, Entity, Render, SharedString, Window};
use std::{cell::Cell, rc::Rc, sync::Arc};

/// The status slider's chosen thumbnail stop, GXDE DStatusBar m_scaleSlider's valueChanged.
pub(crate) struct ThumbSlid(pub usize);
impl gpui::EventEmitter<ThumbSlid> for Footer {}

// DStatusBar m_scaleSlider: a 120 px slider anchored 20 px off the right edge.
const SLIDER_W: f32 = 120.;
const SLIDER_MARGIN: f32 = 20.;

#[path = "status_counts.rs"]
mod counts;
#[path = "status_detail.rs"]
mod detail;
#[path = "status_filesystem.rs"]
mod filesystem;
#[path = "status_footer.rs"]
mod handoff;
pub(crate) use filesystem::Filesystem;

#[cfg(test)]
#[path = "status_paint_tests.rs"]
pub(crate) mod paint_tests;

pub(crate) struct Footer {
    pub line: Entity<MessageLine>,
    pub(super) external_line: Option<Entity<MessageLine>>,
    pub painted: Paint,
    pub path: String,
    pub total: usize,
    pub selected: usize,
    pub kind: Kind,
    pub counts: SharedString,
    pub filesystem: SharedString,
    /// The left zone's text as last laid out, cut with an ellipsis when it outgrows its zone.
    pub counts_text: gpui::TextLayout,
    // The search receipt last handed to the message line, so a frame at rest says nothing new.
    search: String,
    pub border_width: gpui::Pixels,
    pub width: f32,
    pub zone_width: f32,
    /// The thumbnail stop the GXDE-style scale slider shows.
    pub thumb: usize,
    slider_left: Rc<Cell<f32>>,
    slider_drag: bool,
    probe: Option<crate::visual::Probe>,
    revision: u64,
    _changed: gpui::Subscription,
}

impl Footer {
    /// The stop a press inside the slider names: its fraction across the groove, snapped to THUMB_STOPS.
    fn slider_index(&self, x: gpui::Pixels) -> usize {
        let last = crate::grid::geometry::THUMB_STOPS.len() - 1;
        let norm = ((f32::from(x) - self.slider_left.get()) / SLIDER_W).clamp(0., 1.);
        ((norm * last as f32) + 0.5) as usize
    }

    /// DStatusBar m_scaleSlider: a horizontal groove with an accent fill and a circular handle, which drags to a thumbnail stop.
    fn slider(&self, height: f32, ink: &Palette, cx: &mut Context<Self>) -> impl IntoElement {
        let last = (crate::grid::geometry::THUMB_STOPS.len() - 1) as f32;
        let centre = ((self.thumb as f32 / last) * SLIDER_W).clamp(7., SLIDER_W - 7.);
        let left = self.slider_left.clone();
        div()
            .absolute()
            .right(px(SLIDER_MARGIN))
            .top_0()
            .h(px(height))
            .w(px(SLIDER_W))
            .flex()
            .items_center()
            .cursor_pointer()
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(|view, event: &gpui::MouseDownEvent, window, cx| {
                    // The strip keeps the keyboard; the press only moves the handle.
                    window.prevent_default();
                    view.slider_drag = true;
                    let index = view.slider_index(event.position.x);
                    if index != view.thumb {
                        cx.emit(ThumbSlid(index));
                    }
                    cx.notify();
                }),
            )
            .on_mouse_move(cx.listener(|view, event: &gpui::MouseMoveEvent, _, cx| {
                if view.slider_drag {
                    let index = view.slider_index(event.position.x);
                    if index != view.thumb {
                        cx.emit(ThumbSlid(index));
                    }
                }
            }))
            .on_mouse_up(
                gpui::MouseButton::Left,
                cx.listener(|view, _, _, _| view.slider_drag = false),
            )
            .on_mouse_up_out(
                gpui::MouseButton::Left,
                cx.listener(|view, _, _, _| view.slider_drag = false),
            )
            .child(
                div()
                    .relative()
                    .w_full()
                    .h(px(4.))
                    .rounded_full()
                    .bg(ink.foreground.opacity(0.18))
                    .child(
                        gpui::canvas(
                            move |bounds, _, _| left.set(f32::from(bounds.origin.x)),
                            |_, _, _, _| {},
                        )
                        .absolute()
                        .size_full(),
                    )
                    .child(
                        div()
                            .absolute()
                            .left_0()
                            .top_0()
                            .h_full()
                            .w(px(centre))
                            .rounded_full()
                            .bg(ink.accent),
                    )
                    .child(
                        div()
                            .absolute()
                            .top(px(-5.))
                            .left(px(centre - 7.))
                            .size(px(14.))
                            .rounded_full()
                            .bg(if ink.background.l > 0.62 {
                                ink.background
                            } else {
                                ink.foreground
                            })
                            .border_1()
                            .border_color(ink.muted_frame),
                    ),
            )
    }

    pub(crate) fn message_line(&self) -> Entity<MessageLine> {
        self.external_line.as_ref().unwrap_or(&self.line).clone()
    }
    fn new(preset: &str, catalog: Arc<I18n>, cx: &mut Context<Self>) -> Self {
        let line = cx.new(|cx| MessageLine::new(preset, catalog, cx));
        let changed = cx.observe(&line, |_, _, cx| cx.notify());
        Self {
            line,
            external_line: None,
            painted: Default::default(),
            path: String::new(),
            total: 0,
            selected: 0,
            kind: Kind::Loading,
            counts: SharedString::default(),
            filesystem: SharedString::default(),
            counts_text: Default::default(),
            search: String::new(),
            border_width: px(0.),
            width: 0.,
            zone_width: 0.,
            thumb: 1,
            slider_left: Rc::default(),
            slider_drag: false,
            probe: None,
            revision: 0,
            _changed: changed,
        }
    }
}

impl Shell {
    pub(crate) fn footer(&mut self, cx: &mut Context<Self>) -> Entity<Footer> {
        let preset = self
            .launch
            .settings
            .data
            .get("keys")
            .and_then(dlea::jsondoc::Json::as_str)
            .unwrap_or("default");
        let catalog = self.locale.catalog.clone();
        let footer = self.surfaces[0]
            .footer
            .get_or_insert_with(|| cx.new(|cx| Footer::new(preset, catalog, cx)))
            .clone();
        if self.surfaces[0].footer_watch.is_none() {
            let line = self.footer_undo_line(&footer, cx);
            self.surfaces[0].footer_watch = Some(cx.observe(&line, |shell, line, cx| {
                let detail = line.read_with(cx, |line, _| line.drawn().detail.clone());
                if shell.surfaces[0].detail_text != detail {
                    shell.surfaces[0].detail_text = detail;
                    cx.notify();
                }
            }));
            // The notice channel's own clock ends the operation line that a listing keeps.
            let ended = cx.subscribe(&line, |shell, _, _: &NoticeEnded, cx| {
                for host in shell.panes.iter_mut().flatten() {
                    host.end_operation_line();
                }
                cx.notify();
            });
            self.surfaces[0].notice_watch = Some(ended);
            let slider = cx.subscribe(&footer, |shell, _, event: &ThumbSlid, cx| {
                shell.slider_thumb(event.0, cx);
            });
            self.surfaces[0].thumb_watch = Some(slider);
        }
        footer
    }

    /// Producers publish through the same service that the footer paints and the IPC readers observe.
    pub fn status_messages<R>(
        &mut self,
        cx: &mut Context<Self>,
        change: impl FnOnce(&mut Messages, u64) -> R,
    ) -> R {
        self.footer(cx)
            .read_with(cx, |footer, _| footer.line.clone())
            .update(cx, |line, cx| line.update(cx, change))
    }

    pub(crate) fn status_refused(
        &mut self,
        operation: &str,
        message: &str,
        named: &str,
        cx: &mut Context<Self>,
    ) {
        let sentence = dlea::ui_logic_status_feedback::errors::sentence(operation, message, named)
            .render(&self.locale.catalog);
        self.status_messages(cx, |messages, now| {
            messages.say(&sentence, operation != "sort", "", Hint::None, now)
        });
    }

    /// Says a translated sentence in the footer as a note, for input the pane holds or cannot act on.
    pub(crate) fn status_note(&mut self, text: &str, cx: &mut Context<Self>) {
        handoff::note(self, text, cx);
    }

    pub(crate) fn sync_footer(&mut self, width: f32, cx: &mut Context<Self>) {
        let footer = self.footer(cx);
        let Some(host) = &self.panes[self.focus_side] else {
            return;
        };
        let trash = counts::trash(self, cx);
        let in_trash = trash.is_some();
        let flash = trash.is_none().then(|| counts::flash(self)).flatten();
        let in_flash = flash.is_some();
        let trash = trash.or(flash);
        let counts::Snapshot {
            kind,
            total,
            selected,
            text: counts,
        } = trash.unwrap_or_else(|| counts::folder(host, &self.locale.catalog));
        let catalog = &self.locale.catalog;
        let fs = &mut self.surfaces[self.focus_side].filesystem;
        fs.enter(&host.path);
        // RecentMode.js: Recent spans disks and asks for no figures, so its bar answers unknown.
        let recent = host.recent.on || host.computer_items.is_some() || crate::navigation::history::is_computer(host.pane.place()) || crate::navigation::history::is_computer(&host.path);
        if !recent
            && !host.pane.busy()
            && fs.listing != Some(host.pane.held.listing)
            && !host.stopped
        {
            fs.listing = Some(host.pane.held.listing);
            if let Some(backend) = &host.backend {
                let _ = backend.fsinfo();
            }
        }
        let free = fs
            .facts
            .as_ref()
            .and_then(|facts| facts.free)
            .map(|free| catalog.locale().format_size(free));
        let name = fs.facts.as_ref().map_or_else(String::new, |facts| {
            status_host::place_name(catalog, &facts.name, &facts.class, free.is_some())
        });
        let filesystem = if recent {
            status_host::disk(catalog, "", None, host.computer_items.is_some() || crate::navigation::history::is_computer(host.pane.place()) || crate::navigation::history::is_computer(&host.path))
        } else {
            status_host::disk(catalog, &name, free.as_deref(), fs.pending(kind))
        };
        // HIG search.png: the landed receipt stands before the disk facts while search lives.
        let (path, state_message, retry, search) = if in_trash || in_flash {
            (
                catalog.tr(if in_trash {
                    "trash.title"
                } else {
                    "archive.flash.title"
                }),
                String::new(),
                String::new(),
                String::new(),
            )
        } else {
            // StatusBar.qml keyHint compares against the sentence the state slot draws, a refusal's own when there is one.
            let drawn = matches!(
                host.pane.state.kind,
                crate::pane::states::Kind::Error | crate::pane::states::Kind::Locked
            )
            .then(|| self.surfaces[self.focus_side].refused(catalog))
            .flatten();
            (
                host.path.clone(),
                drawn.map_or_else(
                    || host.state_text.message.clone(),
                    |facts| facts.sentence.to_string(),
                ),
                host.retry_text(),
                crate::g2::file_search::Search::status(&host.search.owner, catalog).0,
            )
        };
        let revision = self.locale.revision;
        let catalog = self.locale.catalog.clone();
        let probe = self
            .visual
            .pending
            .as_ref()
            .map(|_| self.visual.probe.clone());
        let thumb = host
            .grid
            .as_ref()
            .map(|grid| grid.view.thumb.index())
            .unwrap_or(1);
        footer.update(cx, |footer, cx| {
            let changed = footer.path != path
                || footer.total != total
                || footer.selected != selected
                || footer.kind != kind
                || footer.counts.as_ref() != counts
                || footer.filesystem.as_ref() != filesystem
                || footer.width != width
                || footer.revision != revision;
            footer.thumb = thumb;
            footer.path = path.clone();
            footer.total = total;
            footer.selected = selected;
            footer.kind = kind;
            footer.counts = counts.into();
            footer.width = width;
            footer.filesystem = filesystem.into();
            footer.probe = probe;
            footer.zone_width = ((width - 28.).max(0.) / 3.).round();
            footer.line.update(cx, |line, cx| {
                if footer.revision != revision {
                    line.translated(catalog, cx);
                }
                if changed {
                    if let Some(arm) = line.update(cx, |m, now| m.set_path(&path, now)) {
                        cx.emit(ArmLost(arm));
                    }
                }
                line.set_pane_state(&state_message, kind == Kind::Waiting, cx);
                line.set_retry_line(&retry, cx);
                if footer.search != search {
                    line.update(cx, |m, now| m.set_search(&search, now));
                }
            });
            footer.search = search;
            footer.revision = revision;
        });
        self.slots.status = Some(footer.into());
    }

    pub(crate) fn status_escape(&mut self, cx: &mut Context<Self>) -> bool {
        // Focus.js:187-190: a standing filter or search takes Escape before the status line acknowledges an error.
        if self.panes[self.focus_side].as_ref().is_some_and(|host| {
            host.filter.owner.up()
                || host.pane.state.search
                || host.pane.swap.session.open
                || !host.pane.state.query.is_empty()
        }) {
            return false;
        }
        let Some(footer) = &self.surfaces[0].footer else {
            return false;
        };
        let line = footer.read_with(cx, |footer, _| footer.line.clone());
        let error = line.read_with(cx, |line, _| line.messages().shown().is_error);
        if error {
            line.update(cx, |line, cx| line.update(cx, |m, now| m.dismiss(now)));
        }
        error
    }
}

impl Render for Footer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.painted.clear();
        let theme = crate::window::glass::producers::chrome_theme(window, cx);
        let ink = Palette::inherited(&window.text_style(), theme);
        let typeface = Typeface::for_window(window, cx);
        let height = Metrics::for_typeface(&typeface).header;
        let style = gpui::TextStyle {
            font_family: typeface.font.family.clone(),
            font_size: typeface.caption.into(),
            ..window.text_style()
        };
        let measure = |text: &str| {
            let width = window
                .text_system()
                .layout_line(text, typeface.caption, &[style.to_run(text.len())], None)
                .width;
            f32::from(width)
        };
        let disk_width = measure(&self.filesystem).min(self.zone_width);
        // HIG locked-menu lets the full notice use the count zone's unpainted space.
        let count_width = measure(&self.counts).min(self.zone_width);
        // DStatusBar anchors its 120 px scale slider 20 px off the right edge; the disk facts step in before it.
        let disk_x = SLIDER_MARGIN + SLIDER_W + 14.;
        let room = (self.width - 28. - disk_width - disk_x - (14. + count_width + 14.)).max(0.);
        let counts_text = counts::elided(&self.counts, self.zone_width, measure);
        self.counts_text = counts_text.layout().clone();
        // A block zone, not a flex row: only a text given its zone's width is cut with the ellipsis StatusBar.qml draws.
        let text = |name, value: SharedString, child: gpui::AnyElement| {
            let element = self.painted.watch(
                div()
                    .relative()
                    .w(px(self.zone_width))
                    .h(px(height))
                    .line_height(px(height))
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .text_ellipsis()
                    .child(child),
                name,
                value.clone(),
            );
            if name == "counts" {
                crate::visual::geometry::label(
                    element,
                    "status-text".into(),
                    "status".into(),
                    value,
                    0.,
                    false,
                    self.probe.clone(),
                )
            } else {
                element
            }
        };
        let root = div()
            .relative()
            .w_full()
            .h(px(height))
            .flex_none()
            .border(self.border_width)
            // HIG list and rail-favorites: the status bar stands on --surface under its --hair edge.
            .child(super::glass::producers::status_backdrop(theme.surface))
            .font_family(typeface.font.family)
            .text_size(typeface.caption)
            .text_color(ink.muted)
            .child(
                div()
                    .absolute()
                    .top_0()
                    .w_full()
                    .h(px(1.))
                    .bg(ink.foreground.opacity(0.12)),
            )
            .child(
                text(
                    "counts",
                    self.counts.clone(),
                    counts_text.into_any_element(),
                )
                .absolute()
                .left(px(((self.width - self.zone_width) / 2.).max(14.))),
            )
            .child(
                text(
                    "disk",
                    self.filesystem.clone(),
                    self.filesystem.clone().into_any_element(),
                )
                .absolute()
                .right(px(disk_x))
                .text_right(),
            )
            .child(
                self.painted.watch(
                    div()
                        .absolute()
                        .right(px(disk_x + disk_width + 14.))
                        .w(px(room))
                        .h(px(height))
                        .child(self.message_line()),
                    "lane",
                    SharedString::default(),
                ),
            )
            .child(self.slider(height, &ink, cx));
        let footer = div()
            .size_full()
            .flex()
            .flex_col()
            .child(self.painted.watch(root, "frame", SharedString::default()))
            .children(
                self.message_line()
                    .read_with(cx, |line, cx| line.detail(window, cx)),
            );
        #[cfg(test)]
        let footer = paint_tests::watch(footer, self, cx);
        footer
    }
}
