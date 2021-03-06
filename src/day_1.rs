const DESIRED_SUM: i32 = 2020;

#[allow(dead_code)]
const PT1_EXAMPLE_DESIRED_PRODUCT: i32 = 514579;

#[allow(dead_code)]
const PT2_EXAMPLE_DESIRED_PRODUCT: i32 = 241861950;

#[allow(dead_code)]
const PT1_EXAMPLE_INPUT: [i32; 6] = [
    1721,
    979,
    366,
    299,
    675,
    1456,
];

const SAMPLE_INPUT: [i32; 200] = [
    1864,
    1880,
    1300,
    1961,
    1577,
    1900,
    1307,
    1818,
    1736,
    1846,
    1417,
    1372,
    1351,
    1860,
    1738,
    1525,
    1798,
    1218,
    1723,
    1936,
    1725,
    1998,
    1466,
    1922,
    1782,
    1947,
    1717,
    1914,
    1843,
    1732,
    1918,
    814,
    1771,
    1712,
    1804,
    1213,
    1859,
    1820,
    1793,
    1870,
    1993,
    1787,
    1824,
    1849,
    1646,
    1489,
    1348,
    1978,
    1628,
    1781,
    2002,
    1297,
    1829,
    1596,
    1819,
    1313,
    1413,
    1726,
    1449,
    1810,
    1295,
    1679,
    1358,
    1949,
    1644,
    1825,
    1891,
    490,
    1962,
    1939,
    1228,
    1889,
    1977,
    1980,
    1763,
    1752,
    1983,
    1785,
    1678,
    2000,
    1857,
    1658,
    1863,
    1330,
    1380,
    1799,
    1789,
    1633,
    1663,
    296,
    1985,
    1117,
    1239,
    1854,
    1960,
    2004,
    1940,
    1876,
    1739,
    1858,
    1283,
    1423,
    1982,
    1836,
    1451,
    1840,
    1347,
    1652,
    1695,
    1210,
    1861,
    1199,
    1346,
    1786,
    1814,
    1958,
    1853,
    1974,
    1917,
    1308,
    654,
    1743,
    1847,
    1367,
    1559,
    1614,
    1897,
    2003,
    1886,
    1885,
    1682,
    1204,
    1986,
    1816,
    1994,
    1817,
    1751,
    1701,
    1619,
    1970,
    816,
    1852,
    1832,
    1631,
    703,
    1604,
    1444,
    1842,
    1984,
    1259,
    1948,
    1620,
    1681,
    1822,
    1865,
    1521,
    1741,
    1455,
    1909,
    1764,
    261,
    1464,
    1905,
    1325,
    1766,
    1749,
    1292,
    1874,
    1267,
    1269,
    1969,
    1991,
    1219,
    1345,
    1976,
    1369,
    1942,
    1388,
    1776,
    1629,
    1987,
    1684,
    1813,
    1203,
    1965,
    1729,
    1930,
    1609,
    1801,
    1402,
    121,
    1833,
    1898,
    1957,
    1051,
    1430,
    1893,
    1784,
    1800,
    1910,
];

#[allow(dead_code)]
/// Computes the solution to first part with the sample input.
pub fn pt1_solution() -> i32 {
   let (first, second) = find_pairwise_sum(
       &SAMPLE_INPUT.to_vec(),
       DESIRED_SUM,
    ).unwrap();
   
   first * second
}

#[allow(dead_code)]
/// Computes the solution to the second part with the sample input.
pub fn pt2_solution() -> i32 {
    let (first, second, third) = find_tripletwise_sum(
        &SAMPLE_INPUT.to_vec(),
        DESIRED_SUM,
    ).unwrap();

    first * second * third
}

#[allow(dead_code)]
/// Attempts to find a pair of numbers in a collection that sum to a
/// particular value.
fn find_pairwise_sum(haystack: &Vec<i32>, needle: i32) -> Option<(i32, i32)>
{
    let mut collection: Vec<i32> = haystack.clone();

    collection.sort();

    for first in haystack.iter() {
        let second = needle - first;

        if let Ok(_) = collection.binary_search(&second) {
            return Some((*first, second));
        }
    }

    None
}

#[allow(dead_code)]
/// Attempts to find a triplet of numbers in a collection that sum
/// to a particular value.
fn find_tripletwise_sum(
    haystack: &Vec<i32>,
    needle: i32
) -> Option<(i32, i32, i32)>
{
    let mut collection: Vec<i32> = haystack.clone();

    collection.sort();

    for first in haystack.iter() {
        let possible_second_values = collection.iter()
            .filter(|&value| *first + value <= needle);

        for second in possible_second_values {
            let third = needle - *first - *second;

            if let Ok(_) = collection.binary_search(&third){
                return Some((*first, *second, third));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_can_solve_example() {
        match find_pairwise_sum(&PT1_EXAMPLE_INPUT.to_vec(), DESIRED_SUM) {
            Some((first, second)) => {
                assert_eq!(first + second, DESIRED_SUM);
                assert_eq!(first * second, PT1_EXAMPLE_DESIRED_PRODUCT);
            },

            None => assert!(false),
        }
    }

    #[test]
    fn pt1_can_solve_sample_input() {
        match find_pairwise_sum(&SAMPLE_INPUT.to_vec(), DESIRED_SUM) {
            Some((first, second)) => assert_eq!(first + second, DESIRED_SUM),

            None => assert!(false),
        }
    }

    #[test]
    fn pt2_can_solve_example() {
        match find_tripletwise_sum(&PT1_EXAMPLE_INPUT.to_vec(), DESIRED_SUM) {
            Some((first, second, third)) => {
                assert_eq!(first + second + third, DESIRED_SUM);
                assert_eq!(first * second * third, PT2_EXAMPLE_DESIRED_PRODUCT);
            },

            None => assert!(false),
        }
    }

    #[test]
    fn pt2_can_solve_sample_input() {
        match find_tripletwise_sum(&SAMPLE_INPUT.to_vec(), DESIRED_SUM) {
            Some((first, second, third)) =>
                assert_eq!(first + second + third, DESIRED_SUM),

            None => assert!(false),
        }
    }
}
