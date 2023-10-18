pub enum OperationType{
    Adddition,
    Substraction,
    Multipling,
    Dividing,
}

pub struct Calc{
    roman_nums_map: Vec<(String, i8)>
}

impl Calc{
    
    pub fn new() -> Calc{
        let mut temp_map: Vec<(String, i8)> = Vec::new();

        temp_map.push((String::from("C"), 100));
        temp_map.push((String::from("L"), 50));
        temp_map.push((String::from("X"), 10));
        temp_map.push((String::from("V"), 5));
        temp_map.push((String::from("I"), 1));

        Calc{
            roman_nums_map: temp_map,
        }
    }
    
    pub fn calculate(&self, f_number: i8, s_number: i8, operation: OperationType) -> i8 {
        match operation{
            OperationType::Adddition => f_number + s_number,
            OperationType::Substraction => f_number - s_number,
            OperationType::Multipling => f_number * s_number,
            OperationType::Dividing => f_number / s_number
        }
    }
    
    pub fn verify_number(&self, num: i8) -> bool {
        if (num < 1) || (num > 10) {
            return false;
        }
        true
    }

    pub fn convert_to_roman_nums(&self, mut witness: i8) -> String{
        let mut roman_output = String::new();

        if witness == 0{
            return "null".to_string();
        }
        if witness < 0{
            witness = witness.abs();
            roman_output = roman_output + "-";
        }

        for (key, value) in &self.roman_nums_map {
            if (value == &100) || (value == &50) {
                while (witness - value) >= 0 {
                    witness -= value;
                    roman_output = roman_output + key;
                }
                // XC XL
                if (value - witness) <= 10 {
                    roman_output = roman_output + "X" + key;
                    witness = witness + 10 - value;
                }
                // XC XL
            }
            else {
                while (witness - value) >= 0 {
                    witness -= value;
                    roman_output = roman_output + key;
                }
                // IX IV 
                if ((value - witness) == 1) && (witness != 0)
                {
                    witness = witness + 1 - value;
                    roman_output = roman_output + "I" + key;
                }
                //  IX IV 
            }
        }
        return roman_output;
    }
}