use crate::filter;
use crate::utils;

pub struct Parser {
    pub parsed_data: Option<Vec<Vec<Vec<f64>>>>,
    data: String,
}
impl Parser {
    pub fn new(data: &str) -> Self {
        Self {
            data: data.to_string(),
            parsed_data: None,
        }
    }

    pub fn display(&self) {
        println!("Parsed data: {:#?}", self.parsed_data);
    }

    pub fn run(data: &str) -> Self {
        let mut parser = Parser::new(data);
        parser.parse();
        parser
    }

    fn parse(&mut self) {
        let raw_data = self
            .data
            .split(';')
            .map(|s| s.split('|'))
            .map(|s| {
                s.map(|s| {
                    s.split(',')
                        .map(|s| s.parse::<f64>().unwrap())
                        .collect::<Vec<f64>>()
                })
                .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        if raw_data[0][0].len() != 3 {
            panic!("Invalid data");
        }

        if raw_data[0].len() == 1 {
            let raw_data_len = raw_data.len();
            // [
            //    [ x1, x2, x3 ],
            //    [ y1, y2, y3 ],
            //    [ z1, z2, z3 ],
            //]
            let components = utils::transpose(raw_data.into_iter().flatten().collect());

            // [
            //    [ [x1_user, x2_user, x3_user], [x1_gravity, x2_gravity, x3_gravity] ],
            //    [ [y1_user, y2_user, y3_user], [y1_gravity, y2_gravity, y3_gravity] ],
            //    [ [z1_user, z2_user, z3_user], [z1_gravity, z2_gravity, z3_gravity] ],
            //]
            let split_components = components
                .into_iter()
                .map(|total_acceleration| {
                    let filter = filter::Filter;
                    let gravity_acceleration = filter.filter_low_0_hz(&total_acceleration);
                    let user_acceleration = total_acceleration
                        .iter()
                        .zip(gravity_acceleration.iter())
                        .map(|(total, gravity)| total - gravity)
                        .collect::<Vec<f64>>();
                    vec![user_acceleration, gravity_acceleration]
                })
                .collect::<Vec<_>>();

            // [
            //    [ [x1_user, y1_user, z1_user], [x1_gravity, y1_gravity, z1_gravity] ],
            //    [ [x2_user, y2_user, z2_user], [x2_gravity, y2_gravity, z2_gravity] ],
            //    [ [x3_user, y3_user, z3_user], [x3_gravity, y3_gravity, z3_gravity] ],
            //]
            let standard_format_data = (0..raw_data_len).map(|i| {
                let user = split_components
                    .iter()
                    .map(|component| component.first().unwrap())
                    .map(|component| component[i])
                    .collect::<Vec<f64>>();
                let gravity = split_components
                    .iter()
                    .map(|component| component.last().unwrap())
                    .map(|component| component[i])
                    .collect::<Vec<f64>>();
                vec![user, gravity]
            });

            self.parsed_data = Some(standard_format_data.collect());
        } else if raw_data[0].len() == 2 {
            // Raw data is already in standard format
            self.parsed_data = Some(raw_data);
        } else {
            panic!("Invalid data");
        }
    }
}
