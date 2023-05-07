use crate::preprocessing::Passenger;



pub fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}


pub fn logistic_regression(passengers: &[Passenger], learning_rate: f64, num_iterations: usize) -> Vec<f64> {
    let mut weights = vec![0.0, 0.0, 0.0, 0.0];
    let m = passengers.len() as f64;

    for _ in 0..num_iterations {
        let mut gradients = vec![0.0, 0.0, 0.0, 0.0];
        for passenger in passengers {
            let z = weights[0] * passenger.survived
                + weights[1] * passenger.pclass
                + weights[2] * passenger.sex
                + weights[3] * passenger.age;
            let h = sigmoid(z);
            let y = passenger.survived;

            gradients[0] += (h - y) * passenger.survived;
            gradients[1] += (h - y) * passenger.pclass;
            gradients[2] += (h - y) * passenger.sex;
            gradients[3] += (h - y) * passenger.age;
        }
        for i in 0..weights.len() {
            weights[i] -= learning_rate * gradients[i] / m;
        }
    }

    weights
}

pub fn make_prediction(weights: &[f64], passenger: &Passenger) -> f64 {
        let z = weights[0] * passenger.survived
            + weights[1] * passenger.pclass
            + weights[2] * passenger.sex
            + weights[3] * passenger.age;
        sigmoid(z)
    }
// Add this at the end of the logistic_regression.rs file

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preprocessing::read_data;

    fn create_test_passengers() -> Vec<Passenger> {
        vec![
            Passenger {
                survived: 1.0,
                pclass: 1.0,
                sex: 0.0,
                age: 25.0,
            },
            Passenger {
                survived: 0.0,
                pclass: 3.0,
                sex: 1.0,
                age: 30.0,
            },
        ]
    }

    #[test]
    fn test_sigmoid() {
        let z = 0.0;
        let expected = 0.5;
        let result = sigmoid(z);
        assert_eq!(result, expected, "sigmoid(0.0) should return 0.5");
    }

    #[test]
    fn test_logistic_regression() {
        let passengers = create_test_passengers();
        let learning_rate = 0.01;
        let num_iterations = 100;

        let weights = logistic_regression(&passengers, learning_rate, num_iterations);
        assert_eq!(
            weights.len(),
            4,
            "logistic_regression should return a vector of length 4"
        );
    }

    #[test]
    fn test_make_prediction() {
        let weights = vec![0.0, 0.0, 0.0, 0.0];
        let passenger = &create_test_passengers()[0];

        let prediction = make_prediction(&weights, passenger);
        assert!(
            prediction >= 0.0 && prediction <= 1.0,
            "make_prediction should return a value between 0 and 1"
        );
    }
}


