//! Headless frames of the GXDE toolbar. Set FLEA_GXDE_SHOTS to a directory to write PNGs.
use super::startup::View;
use crate::palette::Theme;
use gpui::{px, size, AppContext, HeadlessAppContext, TextRenderingMode};
use image::ImageFormat;
use std::{borrow::Cow, path::PathBuf, sync::Arc};

const GXDE: &str = "background='#202020'\nforeground='#ffffff'\naccent='#2ca7f8'\ndark_background='#252525'\nmuted='#bdbdbd'";
const GXDE_LIGHT: &str = "background='#ffffff'\nforeground='#000000'\naccent='#2ca7f8'\ndark_background='#f7f7f8'\nmuted='#606060'";

fn app() -> HeadlessAppContext {
    let mut app = HeadlessAppContext::with_platform(
        gpui_platform::current_platform(true).text_system(),
        Arc::new(()),
        gpui_platform::current_headless_renderer,
    );
    app.update(|cx| {
        cx.set_global(Theme::from_text(GXDE));
        cx.set_text_rendering_mode(TextRenderingMode::Grayscale);
        cx.text_system()
            .add_fonts(vec![
                Cow::Borrowed(
                    include_bytes!("../../tests/fonts/JetBrainsMono-Regular.ttf").as_slice(),
                ),
                Cow::Borrowed(
                    include_bytes!("../../tests/fonts/JetBrainsMono-Bold.ttf").as_slice(),
                ),
            ])
            .unwrap();
    });
    app
}

fn save(image: &image::RgbaImage, name: &str) {
    let Some(dir) = std::env::var_os("FLEA_GXDE_SHOTS") else {
        return;
    };
    let path = PathBuf::from(dir).join(name);
    image
        .save_with_format(&path, ImageFormat::Png)
        .expect("png");
}

fn rounded_toolbar(window: &gpui::Window) -> bool {
    window.painted_quads().iter().any(|quad| {
        let radius = quad.corner_radii.top_left.0;
        let top = quad.bounds.origin.y.0;
        radius > 3. && radius < 12. && top < 80.
    })
}

#[test]
fn gxde_toolbar_paints_rounded_plates() {
    let mut app = app();
    let (backend, _requests) = crate::backend::Backend::test_channel();
    let held: std::rc::Rc<std::cell::RefCell<Option<gpui::Entity<crate::state::Shell>>>> =
        std::rc::Rc::new(std::cell::RefCell::new(None));
    let slot = held.clone();
    let handle = app
        .open_window(size(px(1100.), px(700.)), move |window, cx| {
            window.set_scale_factor(1.);
            window.set_rem_size(px(14.));
            let shell = cx.new(|cx| crate::state::tests::shell_with(backend, cx));
            *slot.borrow_mut() = Some(shell.clone());
            cx.new(|_| crate::window::root::Root { shell })
        })
        .unwrap();
    let shell = held.borrow().clone().expect("shell");
    app.run_until_parked();
    app.update_window(handle.into(), |_, window, cx| {
        window.refresh();
        window.draw(cx).clear(cx);
        assert!(
            rounded_toolbar(window),
            "the toolbar paints 4 px GXDE plates"
        );
    })
    .unwrap();
    let list = app.capture_screenshot(handle.into()).expect("list frame");
    save(&list, "gxde-list.png");
    assert!(list.width() > 100 && list.height() > 100);

    app.update_window(handle.into(), |_, window, cx| {
        shell.update(cx, |shell, cx| shell.set_view(View::Grid, window, cx));
        window.refresh();
        window.draw(cx).clear(cx);
    })
    .unwrap();
    let grid = app.capture_screenshot(handle.into()).expect("grid frame");
    save(&grid, "gxde-grid.png");

    app.update(|cx| cx.set_global(Theme::from_text(GXDE_LIGHT)));
    app.update_window(handle.into(), |_, window, cx| {
        shell.update(cx, |shell, cx| shell.set_view(View::List, window, cx));
        window.refresh();
        window.draw(cx).clear(cx);
    })
    .unwrap();
    let light = app.capture_screenshot(handle.into()).expect("light frame");
    save(&light, "gxde-list-light.png");
}
