struct Coefficients {
    alpha: [f64; 3],
    beta: [f64; 3],
}

pub struct Filter;
impl Filter {
    const COEFFICIENTS_LOW_0_HZ: Coefficients = Coefficients {
        alpha: [1.0, -1.979133761292768, 0.979521463540373],
        beta: [
            0.000086384997973502,
            0.000172769995947004,
            0.000086384997973502,
        ],
    };
    const COEFFICIENTS_LOW_5_HZ: Coefficients = Coefficients {
        alpha: [1.0, -1.80898117793047, 0.827224480562408],
        beta: [0.095465967120306, -0.172688631608676, 0.095465967120306],
    };
    const COEFFICIENTS_HIGH_1_HZ: Coefficients = Coefficients {
        alpha: [1.0, -1.905384612118461, 0.910092542787947],
        beta: [0.953986986993339, -1.907503180919730, 0.953986986993339],
    };

    pub fn filter_low_0_hz(&self, data: &[f64]) -> Vec<f64> {
        self.filter(data, &Self::COEFFICIENTS_LOW_0_HZ)
    }
    pub fn filter_low_5_hz(&self, data: &[f64]) -> Vec<f64> {
        self.filter(data, &Self::COEFFICIENTS_LOW_5_HZ)
    }
    pub fn filter_high_1_hz(&self, data: &[f64]) -> Vec<f64> {
        self.filter(data, &Self::COEFFICIENTS_HIGH_1_HZ)
    }

    fn filter(&self, data: &[f64], coefficients: &Coefficients) -> Vec<f64> {
        let mut filtered = vec![0.0; data.len()];
        for i in 2..data.len() {
            filtered[i] = coefficients.alpha[0]
                * (data[i] * coefficients.beta[0]
                    + data[i - 1] * coefficients.beta[1]
                    + data[i - 2] * coefficients.beta[2]
                    - filtered[i - 1] * coefficients.alpha[1]
                    - filtered[i - 2] * coefficients.alpha[2])
        }
        filtered
    }
}
