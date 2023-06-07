use chrono::NaiveDate;

mod parser;

pub type SCWVFile = Vec<SeasonalCompositeWeatherVariableRow>;

#[derive(Debug, PartialEq, Clone)]
pub struct SeasonalCompositeWeatherVariableRow {
    date: NaiveDate,
    sc: f64,
    no: f64,
    nw: f64,
    ne: f64,
    em: f64,
    wm: f64,
    wn: f64,
    ws: f64,
    ea: f64,
    nt: f64,
    se: f64,
    so: f64,
    sw: f64,
}

impl SeasonalCompositeWeatherVariableRow {
    fn new_from_array(date: NaiveDate, values: [f64; 13]) -> Self {
        Self {
            date,
            sc: values[0],
            no: values[1],
            nw: values[2],
            ne: values[3],
            em: values[4],
            wm: values[5],
            wn: values[6],
            ws: values[7],
            ea: values[8],
            nt: values[9],
            se: values[10],
            so: values[11],
            sw: values[12],
        }
    }
}
