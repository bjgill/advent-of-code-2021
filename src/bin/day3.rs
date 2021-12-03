const REPORT_LENGTH: usize = 12;

#[derive(PartialEq, Debug)]
struct Report([bool; REPORT_LENGTH]);

fn parse_reports(input: &str) -> Vec<Report> {
    input
        .split("\n")
        .map(|s| {
            Report(
                s.chars()
                    .take(REPORT_LENGTH)
                    .map(|c| c == '1')
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            )
        })
        .collect()
}

fn summarise(reports: Vec<Report>) -> [u32; REPORT_LENGTH] {
    reports.iter().fold([0; REPORT_LENGTH], |mut acc, r| {
        for i in 0..REPORT_LENGTH {
            acc[i] += r.0[i] as u32;
        }
        acc
    })
}

fn calculate_e_g(reports: Vec<Report>) -> (u32, u32) {
    let report_count = reports.len().try_into().unwrap();
    let summary = summarise(reports);

    let most_common_bits: Vec<bool> = summary
        .into_iter()
        .map(|total| total * 2 >= report_count)
        .collect();

    let epsilon = bits_to_u32(most_common_bits);
    let gamma = 2u32.pow(REPORT_LENGTH as u32) - 1 - epsilon;

    (epsilon, gamma)
}

fn bits_to_u32(bits: Vec<bool>) -> u32 {
    bits.iter()
        .zip((0..REPORT_LENGTH).rev())
        .fold(0, |total, (bit, order)| {
            total + (*bit as u32) * 2u32.pow(order as u32)
        })
}

fn main() {
    let input = std::fs::read_to_string("data/day3.txt").unwrap();

    let (epsilon, gamma) = calculate_e_g(parse_reports(&input));
    println!("{:?}", epsilon * gamma);
}

#[cfg(test)]
mod tests {
    use super::*;

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
            bits_to_u32(vec![
                false, false, false, false, false, false, false, false, false, false, false, true
            ]),
            1
        );
        assert_eq!(
            bits_to_u32(vec![
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
