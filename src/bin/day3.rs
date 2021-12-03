#[derive(PartialEq, Debug, Clone)]
struct Report<const L: usize>([bool; L]);

fn parse_reports<const L: usize>(input: &str) -> Vec<Report<L>> {
    input
        .split("\n")
        .map(|s| {
            Report(
                s.chars()
                    .take(L)
                    .map(|c| c == '1')
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            )
        })
        .collect()
}

fn summarise<const L: usize>(reports: Vec<Report<L>>) -> [u32; L] {
    reports.iter().fold([0; L], |mut acc, r| {
        for i in 0..L {
            acc[i] += r.0[i] as u32;
        }
        acc
    })
}

fn calculate_e_g<const L: usize>(reports: Vec<Report<L>>) -> (u32, u32) {
    let report_count = reports.len().try_into().unwrap();
    let summary = summarise(reports);

    let most_common_bits: Vec<bool> = summary
        .into_iter()
        .map(|total| total * 2 >= report_count)
        .collect();

    let epsilon = bits_to_u32(&most_common_bits);
    let gamma = 2u32.pow(L as u32) - 1 - epsilon;

    (epsilon, gamma)
}

fn bits_to_u32(bits: &[bool]) -> u32 {
    bits.iter()
        .zip((0..bits.len()).rev())
        .fold(0, |total, (bit, order)| {
            total + (*bit as u32) * 2u32.pow(order as u32)
        })
}

fn filter_most_common_value_for_position<const L: usize>(
    reports: Vec<Report<L>>,
    position: usize,
) -> Vec<Report<L>> {
    let total_for_column = reports
        .iter()
        .fold(0, |acc, report| acc + report.0[position] as u32);
    let most_common_value = total_for_column * 2 >= reports.len() as u32;

    reports
        .into_iter()
        .filter(|report| report.0[position] == most_common_value)
        .collect()
}

fn filter_least_common_value_for_position<const L: usize>(
    reports: Vec<Report<L>>,
    position: usize,
) -> Vec<Report<L>> {
    let total_for_column = reports
        .iter()
        .fold(0, |acc, report| acc + report.0[position] as u32);
    let most_common_value = total_for_column * 2 >= reports.len() as u32;

    reports
        .into_iter()
        .filter(|report| report.0[position] != most_common_value)
        .collect()
}

fn get_o2_generator_rating<const L: usize>(reports: Vec<Report<L>>) -> u32 {
    let mut reports = reports;
    for i in 0..L {
        reports = filter_most_common_value_for_position(reports, i);

        if reports.len() == 1 {
            break;
        }
    }

    bits_to_u32(&reports[0].0)
}

fn get_co2_scrubber_rating<const L: usize>(reports: Vec<Report<L>>) -> u32 {
    let mut reports = reports;
    for i in 0..L {
        reports = filter_least_common_value_for_position(reports, i);

        if reports.len() == 1 {
            break;
        }
    }

    bits_to_u32(&reports[0].0)
}

fn main() {
    let input = std::fs::read_to_string("data/day3.txt").unwrap();
    let reports = parse_reports::<12>(&input);

    let (epsilon, gamma) = calculate_e_g(reports.clone());
    println!("{:?}", epsilon * gamma);

    let o2 = get_o2_generator_rating(reports.clone());
    let co2 = get_co2_scrubber_rating(reports);
    println!("{:?}", o2 * co2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW_TEST_REPORT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    fn get_test_reports() -> Vec<Report<5>> {
        parse_reports::<5>(&RAW_TEST_REPORT)
    }

    #[test]
    fn test_power_report() {
        assert_eq!(calculate_e_g::<5>(get_test_reports()), (22, 9));
    }

    #[test]
    fn test_most_common() {
        assert_eq!(
            filter_most_common_value_for_position(
                vec![Report([true, false]), Report([false, false])],
                0
            ),
            vec![Report([true, false])]
        );
    }

    #[test]
    fn test_o2_generator() {
        assert_eq!(get_o2_generator_rating(get_test_reports()), 23);
    }

    #[test]
    fn test_least_common() {
        assert_eq!(
            filter_least_common_value_for_position(
                vec![Report([true, false]), Report([false, false])],
                0
            ),
            vec![Report([false, false])]
        );
    }

    #[test]
    fn test_co2_scrubber() {
        assert_eq!(get_co2_scrubber_rating(get_test_reports()), 10);
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_reports("000000000100"),
            vec![Report([
                false, false, false, false, false, false, false, false, false, true, false, false
            ])]
        );
    }

    #[test]
    fn test_sumarise() {
        assert_eq!(
            summarise(vec![
                Report([
                    false, false, false, false, false, false, false, false, false, true, false,
                    false
                ]),
                Report([
                    false, false, false, false, false, false, false, false, false, true, false,
                    false
                ])
            ]),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0]
        );
    }

    #[test]
    fn test_bits_to_u32() {
        assert_eq!(
            bits_to_u32(&[
                false, false, false, false, false, false, false, false, false, false, false, true
            ]),
            1
        );
        assert_eq!(
            bits_to_u32(&[
                false, false, false, false, false, false, false, false, false, false, true, true
            ]),
            3
        );
    }

    #[test]
    fn test_calculate_e_g() {
        assert_eq!(
            calculate_e_g(vec![
                Report([
                    false, false, false, false, false, false, false, false, false, false, false,
                    true
                ]),
                Report([
                    false, false, false, false, false, false, false, false, false, false, true,
                    true
                ])
            ]),
            (3, 4092)
        );
    }
}
