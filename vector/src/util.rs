pub mod temp_utl {

    pub fn storetemp(temperature: &mut Vec<f32>, new_temp:f32) {
        temperature.push(new_temp);
    }

    pub fn initsize() -> Vec<f32> {
        let mut temperature = Vec::new();
        temperature.push(32.0);
        temperature.push(31.5);
        temperature.push(33.0);
        temperature.push(29.8);
        temperature.push(30.2);
        return temperature;
    }

    pub fn computeaverage(temperature: &Vec<f32>) -> f32 {
        let sum: f32 = sum(temperature);
        let count = temperature.len() as f32;
        return sum / count;
    }

    pub fn sum(temperature: &Vec<f32>) -> f32 {
        let mut sum = 0.0;
        for temp in temperature {
            sum += *temp;
        }
        return sum;
    }
}