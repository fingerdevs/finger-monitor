// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]
use druid::im::Vector;
use druid::widget::{prelude::*, FillStrat, Image};
use druid::widget::{Flex, SizedBox, WidgetExt};
use druid::{AppLauncher, Color, Data, ImageBuf, Lens, WindowDesc};

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let root = BitMapBackend::new("0.png", (640, 480)).into_drawing_area();
//     root.fill(&WHITE)?;
//     let mut chart = ChartBuilder::on(&root)
//         .caption("y=x^2", ("Roboto Mono", 50).into_font())
//         .margin(5)
//         .x_label_area_size(30)
//         .y_label_area_size(30)
//         .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

//     chart.configure_mesh().draw()?;

//     chart
//         .draw_series(LineSeries::new(
//             (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
//             &RED,
//         ))?
//         .label("y = x^2")
//         .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

//     chart
//         .configure_series_labels()
//         .background_style(&WHITE.mix(0.8))
//         .border_style(&BLACK)
//         .draw()?;

//     root.present()?;

//     Ok(())
// }

#[derive(Clone, Data, Lens)]
struct AppState {
    cpu_usage: f64,
    cpu_cores_usage: Vector<f64>,
}

/// builds a child Flex widget from some parameters.
struct Rebuilder {
    inner: Box<dyn Widget<AppState>>,
}

impl Rebuilder {
    fn new() -> Rebuilder {
        Rebuilder {
            inner: SizedBox::empty().boxed(),
        }
    }

    fn rebuild_inner(&mut self, data: &AppState) {
        self.inner = build_widget(data);
    }
}

impl Widget<AppState> for Rebuilder {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.rebuild_inner(data);
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, _env: &Env) {
        if !old_data.same(data) {
            self.rebuild_inner(data);
            ctx.children_changed();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.inner.paint(ctx, data, env)
    }

    fn id(&self) -> Option<WidgetId> {
        self.inner.id()
    }
}

fn build_widget(_state: &AppState) -> Box<dyn Widget<AppState>> {
    let png_data = ImageBuf::from_data(include_bytes!("../0.png")).unwrap();

    let img = Image::new(png_data).fill_mode(FillStrat::Fill);
    let sized = SizedBox::new(img);
    sized.border(Color::grey(0.6), 2.0).center().boxed()
}

fn make_ui() -> impl Widget<AppState> {
    Flex::column()
        .must_fill_main_axis(true)
        .with_default_spacer()
        .with_flex_child(Rebuilder::new().center(), 1.0)
        .padding(10.0)
}

fn run_app() {
    let main_window = WindowDesc::new(make_ui())
        .window_size((650., 450.))
        .title("Flex Container Options");

    let state = AppState {
        cpu_usage: 0.54,
        cpu_cores_usage: Vector::new(),
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(state)
        .expect("Failed to launch application");
}

pub fn main() {
    run_app()
}
