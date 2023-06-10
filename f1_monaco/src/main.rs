use polars::frame::DataFrame;
use csv::Writer; 



fn main() {
    let f1_qualifying = DataFrame::default(); 
    assert!(f1_qualifying.is_empty()); 

    let mut wtr = Writer::from_path("f1_qualifying.csv").expect("Unable to create csv file");

   // let s1 = Series::new("Driver", &["Max Verstappen", "Fernando Alonso", "Charles Leclerc", "Estaben Ocon", "Carlos Sainz", "Lewis Hamilton", "Pierre Gasly", "George Russell", "Yuki Tsunoda", "Landi Norris", "Osar Piastri", "Nyck De Vries", "Alexander Albon", "Lance Stroll", "Valtteri Bottas", "Logan Sargeant", "Kevin Magnuessen", "Nico Hulkenberg", "Zhou Gaunyu", "Sergio Perez"]);
   // let s2 = Series::new("QPosition", &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13 ,14, 15, 16, 17, 18, 19, 20]);
   // let s3 = Series::new("RacePosition", &[1, 2, 6, 3, 8, 4, 7, 5, 15, 9, 10, 12, 14, 20, 11, 18, 19, 17, 13, 16]);
   // let f1_qualifying = DataFrame::new(vec![s1, s2,s3]);

    wtr.write_record(&["Driver", "Qualifying", "Position", "delta", "Time"]).expect("Unable to write record");
    wtr.write_record(&["Max Verstappen", "1", "1", "0", "100"]).expect("Unable to write record");
    wtr.write_record(&["Fernando Alonso", "2", "2", "0", "95"]).expect("Unable to write record");
    wtr.write_record(&["Charles Leclerc", "3", "6","-3", "90"]).expect("Unable to write record");
    wtr.write_record(&["Estaben Ocon", "4", "3", "1", "85"]).expect("Unable to write record");
    wtr.write_record(&["Carlos Sainz", "5", "8", "-3", "80"]).expect("Unable to write record");
    wtr.write_record(&["Lewis Hamilton", "6", "4", "2", "75"]).expect("Unable to write record");
    wtr.write_record(&["Pierre Gasly", "7", "7", "0", "70"]).expect("Unable to write record");
    wtr.write_record(&["George Russell", "8", "5", "3", "65"]).expect("Unable to write record");
    wtr.write_record(&["Yuki Tsunoda", "9", "15", "-6", "60"]).expect("Unable to write record");
    wtr.write_record(&["Landi Norris", "10", "9", "1", "55"]).expect("Unable to write record");
    wtr.write_record(&["Osar Piastri", "11", "10", "1", "50"]).expect("Unable to write record");
    wtr.write_record(&["Nyck De Vries", "12", "12", "0", "45"]).expect("Unable to write record");
    wtr.write_record(&["Alexander Albon", "13", "14", "-1", "40"]).expect("Unable to write record");
    wtr.write_record(&["Lance Stroll", "14", "20", "6", "35"]).expect("Unable to write record");
    wtr.write_record(&["Valtteri Bottas", "15", "11", "3", "30"]).expect("Unable to write record");
    wtr.write_record(&["Logan Sargeant", "16", "18", "-2", "25"]).expect("Unable to write record");
    wtr.write_record(&["Kevin Magnuessen", "17", "19", "-2", "20"]).expect("Unable to write record");
    wtr.write_record(&["Nico Hulkenberg", "18", "17", "1", "15"]).expect("Unable to write record");
    wtr.write_record(&["Zhou Gaunyu", "19", "13", "7", "10"]).expect("Unable to write record");
    wtr.write_record(&["Sergio Perez", "20", "16", "4", "5"]).expect("Unable to write record");

    wtr.flush().expect("Unable to flush csv file"); 
  
   // println!("{:?}", f1_qualifying);    

    




}
