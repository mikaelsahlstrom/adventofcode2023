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
    let valid_numbers = vec![("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];
    let mut sum = 0;

    if let Ok(lines) = utils::read_lines("src/day1/input")
    {
        for line in lines
        {
            if let Ok(l) = line
            {
                let mut first_valid_number_pos = valid_numbers.iter().map(|x| (l.find(x.0), x.1)).filter(|x| x.0.is_some()).map(|x| (x.0.unwrap(), x.1)).collect::<Vec<(usize, i32)>>();

                let first_digit_pos = l.find(char::is_numeric);
                if first_digit_pos.is_some()
                {
                    first_valid_number_pos.push((first_digit_pos.unwrap(), l.chars().nth(first_digit_pos.unwrap()).unwrap().to_digit(10).unwrap() as i32));
                }

                first_valid_number_pos.sort_by(|a, b| a.0.cmp(&b.0));

                let first = first_valid_number_pos[0].1;

                let mut second_valid_number_pos = valid_numbers.iter().map(|x| (l.rfind(x.0), x.1)).filter(|x| x.0.is_some()).map(|x| (x.0.unwrap(), x.1)).collect::<Vec<(usize, i32)>>();

                let second_digit_pos = l.rfind(char::is_numeric);
                if second_digit_pos.is_some()
                {
                    second_valid_number_pos.push((second_digit_pos.unwrap(), l.chars().nth(second_digit_pos.unwrap()).unwrap().to_digit(10).unwrap() as i32));
                }

                second_valid_number_pos.sort_by(|a, b| b.0.cmp(&a.0));

                let second = second_valid_number_pos[0].1;

                sum += first * 10 + second;
            }
        }
    }

    println!("Part 2: {}", sum);
}
