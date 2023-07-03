/*
----> ЗАДАНИЕ 1 "Поиск слова в строке"

Вывести номер строки в котором встречается нужное слово и саму строку в формате:
номер строки: строка...

 */

const SEARCH_TERM: &str = "picture";
const QUOTE: &str = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";


fn main() {
    find_term(SEARCH_TERM, QUOTE);
}

fn find_term(search_term: &str, quote: &str) -> String {
    let mut num_of_str = 1; //номер строки
	//let mut str_cnt = 0; //положение курсора
    //let mut asdfg = 0; //символ начала строки
	let mut answer: String = String::from("");
	//for q in quote.chars() {
	//    str_cnt += 1;
	for line in quote.lines() {
	//    if q == '\n' {
    //        let qwerty = &quote[asdfg..str_cnt]; //обрезанная строка
    //        if qwerty.contains(search_term) {
		if line.contains(search_term) {
				if answer == "" {
					//answer = num_of_str.to_string() + ": " + qwerty;
					answer = num_of_str.to_string() + ": " + &line.to_string();
				} else {
					let answer1 = "\n".to_string() + &num_of_str.to_string() + ": " + &line.to_string();  
					answer += &answer1;
				}
		}
			num_of_str += 1;
			//asdfg = str_cnt;
	}
	return answer.to_string();
}
			



//----> TESTS
#[cfg(test)]
mod tests {
    use crate::find_term;
    use crate::{SEARCH_TERM, QUOTE};

    #[test]
    fn correct_line() {
        let answer = find_term(SEARCH_TERM, QUOTE);

        assert_eq!("2: dark square is a picture feverishly turned--in search of what?", answer)
    }
}
