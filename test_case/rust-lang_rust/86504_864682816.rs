rust
impl DataRow {
    /// A function for creating new `DataRow` structs from a vector.
    fn new(line: Vec<&str>) -> DataRow {
        DataRow {
            pruid: if line[0] == "" {0} else {line[0].parse().unwrap()},
            prname: String::from(line[1]),
            prname_fr: String::from(line[2]),
            date: String::from(line[3]),
            numconf: if line[4] == "" {0} else {line[4].parse().unwrap()},
            numprob: if line[5] == "" {0} else {line[5].parse().unwrap()},
            numdeaths: if line[6] == "" {0} else {line[6].parse().unwrap()},
            numtotal: if line[7] == "" {0} else {line[7].parse().unwrap()},
            numtoday: if line[8] == "" {0} else {line[8].parse().unwrap()},
            ratetotal: if line[9] == "" {0.00} else {line[9].parse().unwrap()},
        }
    }

    /// A function for outputting the formatted English-language version of the data.
    fn output_en(&self) -> String {
        let mut line = String::new();

        line.push_str(&col_spacing(self.pruid.to_string(), 8));
        line.push_str(&col_spacing(self.prname.clone(), 30));
        line.push_str(&col_spacing(self.date.clone(), 15));
        line.push_str(&col_spacing(self.numconf.to_string(), 10));
        line.push_str(&col_spacing(self.numprob.to_string(), 10));
        line.push_str(&col_spacing(self.numdeaths.to_string(), 10));
        line.push_str(&col_spacing(self.numtotal.to_string(), 10));
        line.push_str(&col_spacing(self.numtoday.to_string(), 10));
        line.push_str(&col_spacing(self.ratetotal.to_string(), 10));

        line
    }

    /// A function for outputting the formatted French-language version of the data.
    fn output_fr(&self) -> String {
        let mut line = String::new();

        line.push_str(&col_spacing(self.pruid.to_string(), 8));
        line.push_str(&col_spacing(self.prname_fr.clone(), 30));
        line.push_str(&col_spacing(self.date.clone(), 15));
        line.push_str(&col_spacing(self.numconf.to_string(), 10));
        line.push_str(&col_spacing(self.numprob.to_string(), 10));
        line.push_str(&col_spacing(self.numdeaths.to_string(), 10));
        line.push_str(&col_spacing(self.numtotal.to_string(), 10));
        line.push_str(&col_spacing(self.numtoday.to_string(), 10));
        line.push_str(&col_spacing(self.ratetotal.to_string(), 10));

        line
    }
}
