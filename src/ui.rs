use druid::im::Vector;
use druid::piet::ImageFormat;
use druid::widget::{prelude::*, FillStrat, Image};
use druid::widget::{Flex, SizedBox, SvgData, WidgetExt};
use druid::{AppLauncher, Color, Data, ImageBuf, Lens, WindowDesc};

use crate::chart::generate_charts;
use crate::models::SystemInfo;

#[derive(Clone, Data, Lens)]
pub struct FingerState {
    system_info_points: Vector<SystemInfo>,
}

/// builds a child Flex widget from some parameters.
struct Rebuilder {
    inner: Box<dyn Widget<FingerState>>,
}

impl Rebuilder {
    fn new() -> Rebuilder {
        Rebuilder {
            inner: SizedBox::empty().boxed(),
        }
    }

    fn rebuild_inner(&mut self, data: &FingerState) {
        self.inner = build_widget(data);
    }
}

impl Widget<FingerState> for Rebuilder {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut FingerState, env: &Env) {
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &FingerState,
        env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            self.rebuild_inner(data);
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &FingerState,
        data: &FingerState,
        _env: &Env,
    ) {
        if !old_data.same(data) {
            self.rebuild_inner(data);
            ctx.children_changed();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &FingerState,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &FingerState, env: &Env) {
        self.inner.paint(ctx, data, env)
    }

    fn id(&self) -> Option<WidgetId> {
        self.inner.id()
    }
}

fn build_widget(state: &FingerState) -> Box<dyn Widget<FingerState>> {
    let charts = generate_charts(state).expect("Could't generate charts");

    let memory_chart = ImageBuf::from_raw(
        charts.memory_chart.raw,
        ImageFormat::Rgb,
        charts.memory_chart.width,
        charts.memory_chart.height,
    );
    let cpu_chart = ImageBuf::from_raw(
        charts.cpu_chart.raw,
        ImageFormat::Rgb,
        charts.cpu_chart.width,
        charts.cpu_chart.height,
    );

    let memory_img = Image::new(memory_chart).fill_mode(FillStrat::Fill);
    let cpu_img = Image::new(cpu_chart).fill_mode(FillStrat::Fill);
    let mut col = Flex::column();
    col.add_flex_child(memory_img, 1.0);
    col.add_flex_child(cpu_img, 1.0);
    col.border(Color::grey(0.6), 2.0).center().boxed()
}

fn make_ui() -> impl Widget<FingerState> {
    Flex::column()
        .must_fill_main_axis(true)
        .with_default_spacer()
        .with_flex_child(Rebuilder::new().center(), 1.0)
        .padding(10.0)
}

pub fn run_app() {
    let main_window = WindowDesc::new(make_ui())
        .window_size((650., 450.))
        .title("Flex Container Options");

    let state = FingerState {
        system_info_points: Vector::new(),
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(state)
        .expect("Failed to launch application");
}
