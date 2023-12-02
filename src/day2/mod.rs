use crate::utils;

pub fn part1()
{
    let mut sum = 0;

    let amount_red_cubes = 12;
    let amount_green_cubes = 13;
    let amount_blue_cubes = 14;

    if let Ok(lines) = utils::read_lines("src/day2/input")
    {
        for line in lines
        {
            if let Ok(l) = line
            {
                let game = l.split(":").collect::<Vec<&str>>();
                let game_id = game[0].split_whitespace().nth(1).unwrap().parse::<i32>().unwrap();

                let sets = game[1].split(";").collect::<Vec<&str>>();
                let mut valid_sets = 0;
                for set in &sets
                {
                    let mut current_red_cubes = 0;
                    let mut current_green_cubes = 0;
                    let mut current_blue_cubes = 0;

                    let parts = set.split(",").collect::<Vec<&str>>();
                    for part in parts
                    {
                        let part = part.trim().split(" ").collect::<Vec<&str>>();
                        let amount = part[0].parse::<i32>().unwrap();
                        let color = part[1].trim();

                        match color
                        {
                            "red" =>
                            {
                                current_red_cubes += amount;
                            },
                            "green" =>
                            {
                                current_green_cubes += amount;
                            },
                            "blue" =>
                            {
                                current_blue_cubes += amount;
                            },
                            &_ => ()
                        }
                    }

                    if current_red_cubes <= amount_red_cubes && current_green_cubes <= amount_green_cubes && current_blue_cubes <= amount_blue_cubes
                    {
                        valid_sets += 1;
                    }
                }

                if valid_sets == sets.len()
                {
                    sum += game_id;
                }
            }
        }
    }

    println!("Part 1: {}", sum);
}

pub fn part2()
{

}
