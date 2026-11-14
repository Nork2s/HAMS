use std::fs;
use std::iter::Skip;
use std::os::linux::raw::stat;
use std::time::SystemTime;
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

fn init_statistics() -> String {
    let result = fs::read_to_string("./Astronauts/example/statistics.json");
    return result.unwrap()
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
    let mut secs = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    

    while true {
        if secs >= SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() {
            continue;
        }

        let statsFile = init_statistics();
        let mut stats: serde_json::Value = serde_json::from_str(&statsFile).unwrap();

        secs = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        if Path::new("./Astronauts/example/boneDensityData.json").exists(){
            let _ = fs::remove_file("./Astronauts/example/boneDensityData.json");
        }

        
        let daysInSpace = stats["HoursInMicroGravity"].as_i64();
        let daysInInt :i32 = daysInSpace.unwrap().to_string().parse::<i32>().unwrap();
        let monthsInSpace: i32 = daysInInt / 30;
        
        stats["LastUpdate"] = serde_json::json!(secs);
        let json_string = serde_json::to_string_pretty(&stats).expect("Error");
        fs::write("./Astronauts/example/statistics.json",json_string).expect("error");
        for _n in 0..monthsInSpace {
            calculate_bone_density()
        }
    }
}