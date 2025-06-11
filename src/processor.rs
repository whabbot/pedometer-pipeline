use crate::filter;

pub struct Processor {
    data: Vec<Vec<Vec<f64>>>,
    dot_product_data: Option<Vec<f64>>,
    pub filtered_data: Option<Vec<f64>>,
}
impl Processor {
    pub fn new(data: Vec<Vec<Vec<f64>>>) -> Self {
        Self {
            data,
            dot_product_data: None,
            filtered_data: None,
        }
    }

    pub fn display(&self) {
        println!("Dot product data: {:#?}", self.dot_product_data);
        println!("Filtered data: {:#?}", self.filtered_data);
    }

    pub fn run(data: Vec<Vec<Vec<f64>>>) -> Self {
        let mut processor = Processor::new(data);
        processor.dot_product_data();
        processor.filtered_data();
        processor
    }

    fn dot_product_data(&mut self) {
        self.dot_product_data = Some(
            self.data
                .iter()
                .map(|data| {
                    let user = data.first().unwrap();
                    let gravity = data.last().unwrap();
                    user.iter().zip(gravity.iter()).map(|(a, b)| a * b).sum()
                })
                .collect(),
        );
    }

    fn filtered_data(&mut self) {
        let filter = filter::Filter;
        let low_frequency_filtered =
            filter.filter_low_5_hz(&self.dot_product_data.as_ref().unwrap());
        let high_frequency_filtered = filter.filter_high_1_hz(&low_frequency_filtered);
        self.filtered_data = Some(high_frequency_filtered);
    }
}
