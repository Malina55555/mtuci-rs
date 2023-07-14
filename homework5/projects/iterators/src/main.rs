fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
	
	let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
	
	let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
	println!("total is {total}");
	
	let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

//pub trait Iterator {
    //type Item;

    //fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
//}


//fn main() {
  //  let config = Config::build(env::args()).unwrap_or_else(|err| {
    //    eprintln!("Problem parsing arguments: {err}");
      //  process::exit(1);
    //});
    // --snip--
//}

//impl Config {
  //  pub fn build(
    //    mut args: impl Iterator<Item = String>,
    //) -> Result<Config, &'static str> {
      //  args.next();
        //let query = match args.next() {
          //  Some(arg) => arg,
            //None => return Err("Didn't get a query string"),
        //};
        //let file_path = match args.next() {
          //  Some(arg) => arg,
            //None => return Err("Didn't get a file path"),
        //};
        //let ignore_case = env::var("IGNORE_CASE").is_ok();
        //Ok(Config { query, file_path, ignore_case, })
    //}
//}

//pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  //  contents
      //  .lines()
        //.filter(|line| line.contains(query))
        //.collect()
//

//let buffer: &mut [i32];
//let coefficients: [i64; 12];
//let qlp_shift: i16;
//for i in 12..buffer.len() {
  //  let prediction = coefficients.iter()
    //                             .zip(&buffer[i - 12..i])
      //                           .map(|(&c, &s)| c * s as i64)
        //                         .sum::<i64>() >> qlp_shift;
    //let delta = buffer[i];
    //buffer[i] = prediction as i32 + delta;
//}}