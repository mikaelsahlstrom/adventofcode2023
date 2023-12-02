use crate::utils;

pub fn part1()
{
    let mut sum = 0;

    if let Ok(lines) = utils::read_lines("src/day1/input")
    {
        for line in lines
        {
            if let Ok(l) = line
            {
                let first = match l.find(char::is_numeric)
                {
                    Some(pos) =>
                    {
                        l.chars().nth(pos).unwrap().to_digit(10).unwrap()
                    },
                    None =>
                    {
                        continue;
                    }
                };

                let second = match l.rfind(char::is_numeric)
                {
                    Some(pos) =>
                    {
                        l.chars().nth(pos).unwrap().to_digit(10).unwrap()
                    },
                    None =>
                    {
                        continue;
                    }
                };

                sum += first * 10 + second;
            }
        }
    }

    println!("Part 1: {}", sum);
}

pub fn part2()
{
    println!("Part 2");
}
