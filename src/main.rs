use std::fs;
use std::path::Path;

fn init_bone_data() -> (String, String) {
    
    let boneDensityDataFile = fs::read_to_string("./data/bonedata.json") //https://www.nasa.gov/wp-content/uploads/2023/12/ochmo-tb-030-bone-loss.pdf
        .expect("No u dont");

    let AstronautDataFile;//https://www.cdc.gov/nchs/data/series/sr_11/sr11_253.pdf
    
    if Path::new("./Astronauts/example/boneDensityData.json").exists() {
            AstronautDataFile = fs::read_to_string("./Astronauts/example/boneDensityData.json").unwrap();
        } else {
            AstronautDataFile = fs::read_to_string("./Astronauts/example/originalBoneDensityData.json").expect("ERROR IN EXAMPLE BONE DENSITY!");
        };
    
    return (boneDensityDataFile, AstronautDataFile)
}

fn calculate_bone_density() {
    let (boneDensityData, AstronautDataFile) = init_bone_data();
    let BoneJSON: serde_json::Value = serde_json::from_str(&boneDensityData).unwrap();

    let AstronautDataJSON: serde_json::Value = serde_json::from_str(&AstronautDataFile).unwrap();

    let AstronautDataValues = AstronautDataJSON.as_object().unwrap();
    let BoneValues = BoneJSON.as_object().unwrap();

    let mut newValue = AstronautDataValues.clone();

    for ((key1, v1), (_key2, v2)) in AstronautDataValues.iter().zip(BoneValues.iter()) {
        let value = v1.as_f64().unwrap() * v2.as_f64().unwrap();
        
        newValue[key1] = serde_json::json!(v1.as_f64().unwrap() + value);
    }
    let json_string = serde_json::to_string_pretty(&newValue).expect("Error");
    fs::write("./Astronauts/example/boneDensityData.json",json_string).expect("error");
}

fn calculate_counter_meassures() {
    let (boneDensityData, AstronautDataFile) = init_bone_data();
    let AstronautDataJSON: serde_json::Value = serde_json::from_str(&AstronautDataFile).expect("Didnt Work");

    let AstronautDataValues = AstronautDataJSON.as_object().unwrap();
    for (key, v) in AstronautDataValues.iter(){
        println!("{}",v)
    }
}

fn main() {
    if Path::new("./Astronauts/example/boneDensityData.json").exists(){
        let _ = fs::remove_file("./Astronauts/example/boneDensityData.json");
    }
    let daysInSpace = fs::read_to_string("./Astronauts/example/days.txt");
    let daysInInt :i32 = daysInSpace.unwrap().to_string().parse::<i32>().unwrap();
    let monthsInSpace: i32 = daysInInt / 30;
    
    for _n in 0..monthsInSpace {
        calculate_bone_density()
    }
}