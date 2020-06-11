pub struct Rotor {
    map: String,
    notch: Option<char>,
}

impl Rotor {
    pub fn new(map: &str, notch: Option<char>) -> Rotor {
        Rotor { map: String::from(map), notch }
    }
}

pub struct Enigma {
    pub rotors: Vec<Rotor>,
    pub refrector: String,
    pub rotor_offsets: Vec<i32>,
    pub plugboard: String,
    pub log: String,
}

impl Enigma {
    pub fn new(rotors: Vec<Rotor>, refrector: &str) -> Enigma {
        Enigma {
            rotors, refrector: String::from(refrector), rotor_offsets: vec![0; 3], plugboard: String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), log: String::new()
        }
    }

    pub fn encrypt(&mut self, data: &String) -> String {
        fn number_from_latin(character: char) -> i32 {
            character as i32 - 'A' as i32
        }
        fn latin_from_number(number: i32) -> String {
            let temp;
            if number < 0 {
                temp = 26 + (number % 26);
            } else if number > 25 {
                temp = number % 26
            } else {
                temp = number;
            }
            String::from_utf8(vec![temp as u8 + 'A' as u8]).unwrap()
        }
        let data = data.to_uppercase();
        let mut result = String::new();
        let mut ratched = vec![false; self.rotor_offsets.len()];
        let plugboard: Vec<char> = self.plugboard.chars().collect();
        self.log += "IN P 1 2 3 R 3 2 1 Rotors";
        for character in data.chars() {
            if character.is_uppercase() {
                let mut temp = number_from_latin(character);
                self.log += &format!("{}: ", latin_from_number(temp));
                temp = number_from_latin(plugboard[temp as usize]);
                self.log += &format!("{} ", latin_from_number(temp));
                self.rotor_offsets[0] = (self.rotor_offsets[0] + 1) % 26;
                for (i, rotor) in (&self.rotors).into_iter().enumerate() {
                    if let Some(notch) = rotor.notch {
                        if ratched[i] {
                            if i != 0 { self.rotor_offsets[i] = (self.rotor_offsets[i] + 1) % 26 };
                            self.rotor_offsets[i + 1] = (self.rotor_offsets[i + 1] + 1) % 26;
                            ratched[i] = false;
                        }
                        if self.rotor_offsets[i] == number_from_latin(notch) {
                            ratched[i] = true;
                        }
                    }
                    temp = number_from_latin(rotor.map.chars().nth((temp + self.rotor_offsets[i]) as usize % 26).unwrap()) - self.rotor_offsets[i];
                    if temp < 0 {
                        temp = 26 + (temp % 26);
                    } else if temp > 25 {
                        temp = temp % 26;
                    }
                    self.log += &format!("{} ", latin_from_number(temp + self.rotor_offsets[i]));
                }
                temp = number_from_latin(self.refrector.chars().nth(temp as usize).unwrap());
                self.log += &format!("{} ", latin_from_number(temp));
                for (i, rotor) in (&self.rotors).into_iter().rev().enumerate() {
                    temp = rotor.map.chars().into_iter().position(|x| { number_from_latin(x) == (temp + self.rotor_offsets[2 - i]) % 26 }).unwrap() as i32 - self.rotor_offsets[2 - i];
                    if temp < 0 {
                        temp = 26 + (temp % 26);
                    } else if temp > 25 {
                        temp = temp % 26;
                    }
                    self.log += &format!("{} ", latin_from_number(temp + self.rotor_offsets[i]));
                }
                self.log += &format!("[{}, {}, {}]", latin_from_number(self.rotor_offsets[0]), latin_from_number(self.rotor_offsets[1]), latin_from_number(self.rotor_offsets[2]));
                result += &latin_from_number(temp);
            } else {
                result += &character.to_string();
            }
        }
        result
    }
}
