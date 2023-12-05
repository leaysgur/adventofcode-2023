const INPUTS: &str = include_str!("./inputs.txt");

type Maps = Vec<(u64, u64, u64)>;

fn parse_maps(maps: &str) -> Vec<(u64, u64, u64)> {
    maps.lines()
        .map(|line| {
            let mut map = line.split_whitespace();
            (
                map.next().unwrap().parse::<u64>().unwrap(),
                map.next().unwrap().parse::<u64>().unwrap(),
                map.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect::<Maps>()
}

fn resolve_number(n: u64, maps: &Maps) -> u64 {
    for (dest, source, range) in maps {
        if *source <= n && n <= *source + *range {
            return *dest + (n - *source);
        }
    }
    n
}

fn main() {
    // println!("{}", INPUTS);
    let mut inputs = INPUTS.split("\n\n");

    let seeds = inputs
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u64>>();
    // println!("Seeds = {:?}", seeds);

    let seeds_to_soil_maps = parse_maps(inputs.next().unwrap().split_once(":\n").unwrap().1);
    let soil_to_fertilizer_maps = parse_maps(inputs.next().unwrap().split_once(":\n").unwrap().1);
    let fertilizer_to_water_maps = parse_maps(inputs.next().unwrap().split_once(":\n").unwrap().1);
    let water_to_light_maps = parse_maps(inputs.next().unwrap().split_once(":\n").unwrap().1);
    let light_to_temperature_maps = parse_maps(inputs.next().unwrap().split_once(":\n").unwrap().1);
    let temperature_to_humidity_maps = parse_maps(inputs.next().unwrap().split_once(":\n").unwrap().1);
    let humidity_to_location_maps = parse_maps(inputs.next().unwrap().split_once(":\n").unwrap().1);

    let soils = seeds.iter().map(|&seed| resolve_number(seed, &seeds_to_soil_maps)).collect::<Vec<_>>();
    let fertilizers = soils.iter().map(|&soil| resolve_number(soil, &soil_to_fertilizer_maps)).collect::<Vec<_>>();
    let waters = fertilizers.iter().map(|&fertilizer| resolve_number(fertilizer, &fertilizer_to_water_maps)).collect::<Vec<_>>();
    let lights = waters.iter().map(|&water| resolve_number(water, &water_to_light_maps)).collect::<Vec<_>>();
    let temperatures = lights.iter().map(|&light| resolve_number(light, &light_to_temperature_maps)).collect::<Vec<_>>();
    let humidities = temperatures.iter().map(|&temperature| resolve_number(temperature, &temperature_to_humidity_maps)).collect::<Vec<_>>();
    let locations = humidities.iter().map(|&humidity| resolve_number(humidity, &humidity_to_location_maps)).collect::<Vec<_>>();

    let lowest_location_number = locations.iter().min().unwrap();
    println!("Lowest location number: {:?}", lowest_location_number);
}
