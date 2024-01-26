use crate::ui::{
    canvas::{display, rgb::RgbCanvas, Viewport},
    display::{toif::Toif, Color, Font},
    geometry::{Insets, Offset, Point, Rect},
    shape,
    shape::{ProgressiveRenderer, Renderer},
};

use crate::time;
use core::fmt::Write;
use heapless::String;
use static_alloc::Bump;

use crate::{
    trezorhal::io::io_touch_read,
    ui::{
        event::TouchEvent,
        model_tt::theme::bootloader::{FIRE40, REFRESH24, WARNING40},
    },
};

const ICON_GOOGLE: &[u8] = include_res!("model_tt/res/fido/icon_google.toif");

fn draw_component1<'a>(target: &mut impl Renderer<'a>) {
    let r = Rect::from_top_left_and_size(Point::new(30, 120), Offset::new(180, 60));
    shape::Bar::new(r)
        .with_radius(16)
        .with_bg(Color::rgb(96, 128, 128))
        .render(target);

    let r = Rect::from_top_left_and_size(Point::new(50, 50), Offset::new(50, 50));
    shape::Bar::new(r)
        .with_fg(Color::rgb(128, 0, 192))
        .with_bg(Color::rgb(192, 0, 0))
        .with_thickness(4)
        .render(target);

    let r = Rect::new(Point::zero(), Point::new(16, 160));
    shape::Bar::new(r.translate(Offset::new(140, 40)))
        .with_bg(Color::rgb(0, 160, 0))
        .render(target);

    let r = Rect::new(Point::new(0, 0), Point::new(240, 240));
    shape::Text::new(r, "TREZOR!!!")
        .with_fg(Color::rgb(255, 0, 0))
        .render(target);

    let r = Rect::new(Point::new(80, 0), Point::new(240, 240));
    shape::Text::new(r, "TREZOR!!!")
        .with_fg(Color::rgb(0, 255, 0))
        .render(target);

    let r = Rect::new(Point::new(160, 0), Point::new(240, 240));
    shape::Text::new(r, "TREZOR!!!")
        .with_fg(Color::rgb(0, 0, 255))
        .render(target);

    let r = Rect::new(Point::new(80, 80), Point::new(240, 240));
    shape::Text::new(r, "BITCOIN!")
        .with_font(Font::BOLD)
        .render(target);

    let r = Rect::new(Point::new(80, 140), Point::new(240, 240));
    let s = "SatoshiLabs";
    shape::Text::new(r, s)
        .with_fg(Color::rgb(0, 255, 255))
        .render(target);

    shape::Text::new(r.translate(Offset::new(1, 1)), s)
        .with_fg(Color::rgb(255, 0, 0))
        .render(target);

    let pt = Point::new(-1, 40);
    let toif = Toif::new(REFRESH24).unwrap();
    shape::ToifImage::new(pt, toif)
        .with_fg(Color::black())
        .with_bg(Color::white())
        .render(target);

    let pt = Point::new(80, 40);
    let toif = Toif::new(FIRE40).unwrap();
    shape::ToifImage::new(pt, toif).render(target);

    let pt = Point::new(95, 50);
    let toif = Toif::new(WARNING40).unwrap();
    shape::ToifImage::new(pt, toif)
        .with_fg(Color::rgb(64, 192, 200))
        .render(target);

    let pt = Point::new(0, 70);
    let toif = Toif::new(ICON_GOOGLE).unwrap();
    shape::ToifImage::new(pt, toif).render(target);

    let pt = Point::new(120, 120);
    shape::Circle::new(pt, 20)
        .with_bg(Color::white())
        .render(target);
}

fn draw_component2<'a>(target: &mut impl Renderer<'a>) {
    let pt = Point::new(120, 110);
    shape::Circle::new(pt, 60)
        .with_bg(Color::rgb(80, 80, 80))
        .render(target);
    shape::Circle::new(pt, 42)
        .with_bg(Color::rgb(0, 0, 0))
        .with_fg(Color::white())
        .with_thickness(2)
        .render(target);

    let toif = Toif::new(FIRE40).unwrap();
    let icon_tl = Point::new(pt.x - toif.width() / 2, pt.y - toif.height() / 2);
    shape::ToifImage::new(icon_tl, toif).render(target);

    let r = Rect::new(Point::new(35, 190), Point::new(240, 240));
    shape::Text::new(r, "Installing firmware")
        .with_fg(Color::white())
        .render(target);
}

const IMAGE_HOMESCREEN: &[u8] = include_res!("minion.jpg");

fn draw_component3<'a>(target: &mut impl Renderer<'a>) {
    shape::JpegImage::new(Point::new(0, 0), IMAGE_HOMESCREEN).render(target);
}

#[link_section = ".no_dma_buffers"]
static mut POOL: Bump<[u8; 32 * 1024]> = Bump::uninit();

fn draw_screen(split: Point) -> time::Duration {
    let start_time = time::Instant::now();

    let bump = unsafe { &mut *core::ptr::addr_of_mut!(POOL) };
    {
        let mut canvas = display::Canvas::acquire().unwrap();

        let vp = canvas.set_window(canvas.bounds().inset(Insets::new(20, 0, 0, 0)));

        for _ in 1..=1 {
            let mut target =
                ProgressiveRenderer::new(&mut canvas, Some(Color::rgb(0, 0, 48)), bump, 30);

            target.set_viewport(vp.with_origin(Offset::new(split.x, split.y)));
            draw_component1(&mut target);

            target.set_viewport(vp.with_origin(Offset::new(split.x - 240, split.y)));
            draw_component2(&mut target);

            target.set_viewport(vp);

            let r = Rect::new(Point::new(60, 60), Point::new(180, 180));
            //let r = Rect::new(Point::new(0, 0), Point::new(240, 240));
            //Blurring::new(r, 1).render(&mut target);
            shape::Blurring::new(r, 2).render(&mut target);
            //Blurring::new(r, 3).render(&mut target);
            //Blurring::new(r, 4).render(&mut target);
            shape::Bar::new(r)
                .with_fg(Color::white())
                .render(&mut target);

            target.render(16);
        }
    }

    bump.reset();

    time::Instant::now()
        .checked_duration_since(start_time)
        .unwrap()
}

fn draw_info(duration: time::Duration) {
    let bump = unsafe { &mut *core::ptr::addr_of_mut!(POOL) };
    {
        let mut canvas = display::Canvas::acquire().unwrap();

        canvas.set_viewport(Viewport::from_size(Offset::new(240, 20)));

        let blue = Color::rgb(0, 0, 255);
        let yellow = Color::rgb(255, 255, 0);

        let mut target = ProgressiveRenderer::new(&mut canvas, Some(blue), bump, 10);

        let mut info = String::<128>::new();
        write!(info, "time={}ms", duration.to_millis() as f32 / 1.0).unwrap();
        let text = info.as_str();

        let r = Rect::new(Point::new(0, 0), Point::new(240, 240));
        shape::Text::new(r, text)
            .with_fg(yellow)
            .render(&mut target);

        target.render(20);
    }

    bump.reset();
}

#[derive(Copy, Clone)]
struct PocContext {
    split: Point,
    origin: Point,
    delta: Offset,
    pressed: bool,
}

static mut POC_CONTEXT: PocContext = PocContext {
    split: Point::zero(),
    origin: Point::zero(),
    delta: Offset::zero(),
    pressed: false,
};

fn touch_event() -> Option<TouchEvent> {
    let event = io_touch_read();
    if event == 0 {
        return None;
    }
    let event_type = event >> 24;
    let ex = ((event >> 12) & 0xFFF) as i16;
    let ey = (event & 0xFFF) as i16;

    TouchEvent::new(event_type, ex as _, ey as _).ok()
}

#[no_mangle]
extern "C" fn new_drawing_poc() {
    let mut ctx = unsafe { POC_CONTEXT };

    match touch_event() {
        Some(TouchEvent::TouchStart(pt)) => {
            ctx.origin = pt;
        }
        Some(TouchEvent::TouchMove(pt)) => {
            let delta = pt - ctx.origin;
            let k = 2;
            ctx.delta.x = (ctx.delta.x * k + delta.x * (10 - k)) / 10;
            ctx.delta.y = (ctx.delta.y * k + delta.y * (10 - k)) / 10;
        }
        Some(TouchEvent::TouchEnd(_pt)) => {
            ctx.split = ctx.split + ctx.delta;
            ctx.pressed = false;
            ctx.delta = Offset::zero();
        }
        None => {
            if ctx.split.x < 0 {
                ctx.split.x -= ctx.split.x / 4;
            } else if ctx.split.x > 240 {
                ctx.split.x -= (ctx.split.x - 240) / 4;
            }

            if ctx.split.y < -120 {
                ctx.split.y = -120;
            } else if ctx.split.y > 120 {
                ctx.split.y = 120;
            }
        }
    }

    let duration = draw_screen(ctx.split + ctx.delta);
    draw_info(duration);

    unsafe {
        POC_CONTEXT = ctx;
    }
}
