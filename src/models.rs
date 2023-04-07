use druid::{im::Vector, Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct SystemInfo {
    pub cpu_usage: f64,
    pub cpu_cores_usage: Vector<(String, f64)>,

    pub memory_total: u64,
    pub memory_usage: u64,

    pub swap_total: u64,
    pub swap_usage: u64,

    pub timestamp: i64,
}

pub struct Charts {
    pub memory_chart: FingerChart,
    pub cpu_chart: FingerChart,
}

pub struct FingerChart {
    pub raw: Vec<u8>,
    pub height: usize,
    pub width: usize,
}
