use std::time::{Duration, Instant};

use tao::{
    event_loop::{ControlFlow, EventLoopBuilder},
    platform::run_return::EventLoopExtRunReturn,
};
use tray_icon::{
    Icon, MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent,
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
};

/// Shows the tray icon and blocks until the user picks "Quit".
pub fn run(url: &str) -> anyhow::Result<()> {
    let mut event_loop = EventLoopBuilder::new().build();

    let open = MenuItem::new("Open Akoyo", true, None);
    let quit = MenuItem::new("Quit Akoyo", true, None);
    let menu = Menu::new();
    menu.append_items(&[&open, &PredefinedMenuItem::separator(), &quit])?;

    let _tray = TrayIconBuilder::new()
        .with_tooltip("Akoyo")
        .with_icon(icon()?)
        .with_menu(Box::new(menu))
        .with_menu_on_left_click(false)
        .build()?;

    let menu_events = MenuEvent::receiver();
    let tray_events = TrayIconEvent::receiver();

    event_loop.run_return(|_event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(150));

        while let Ok(event) = tray_events.try_recv() {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                crate::open_browser(url);
            }
        }
        while let Ok(event) = menu_events.try_recv() {
            if &event.id == open.id() {
                crate::open_browser(url);
            } else if &event.id == quit.id() {
                *control_flow = ControlFlow::Exit;
            }
        }
    });

    Ok(())
}

/// Draws the tray icon: a violet-to-pink disc with three equalizer bars.
fn icon() -> anyhow::Result<Icon> {
    const SIZE: u32 = 32;
    const BARS: [(u32, f32); 3] = [(9, 10.0), (14, 16.0), (19, 12.0)];

    let center = (SIZE as f32 - 1.0) / 2.0;
    let mut rgba = Vec::with_capacity((SIZE * SIZE * 4) as usize);
    for y in 0..SIZE {
        for x in 0..SIZE {
            let distance = ((x as f32 - center).powi(2) + (y as f32 - center).powi(2)).sqrt();
            let alpha = (center + 0.5 - distance).clamp(0.0, 1.0);

            let on_bar = BARS.iter().any(|&(left, height)| {
                (left..left + 4).contains(&x) && (y as f32 - center).abs() <= height / 2.0
            });
            let [r, g, b] = if on_bar {
                [255, 255, 255]
            } else {
                let t = y as f32 / (SIZE - 1) as f32;
                [lerp(139, 236, t), lerp(92, 72, t), lerp(246, 153, t)]
            };
            rgba.extend_from_slice(&[r, g, b, (alpha * 255.0) as u8]);
        }
    }
    Ok(Icon::from_rgba(rgba, SIZE, SIZE)?)
}

fn lerp(from: u8, to: u8, t: f32) -> u8 {
    (from as f32 + (to as f32 - from as f32) * t).round() as u8
}
