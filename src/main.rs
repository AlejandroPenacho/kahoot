mod central;
mod port;

#[derive(Debug)]
pub struct Question {
    text: String,
    answers: Vec<String>,
    correct: usize
}


#[tokio::main]
async fn main() {
    let questions = read_file("questions/Q1.txt");

    central::run_game(questions).await;
}



fn read_file(path: &str) -> Vec<Question> {
    let file = std::fs::read_to_string(path).unwrap();
    let mut lines = file.lines();

    let mut out = Vec::new();

    loop{

        let mut question: Vec<&str> = lines.by_ref().take(6).collect();
        if question.len() < 6 {return out};

        let correct = question[1..5].iter()
            .enumerate()
            .find(|(_,ans)| ans.chars().last() == Some('*'))
            .expect("Question with no correct answer").0;

        question[correct+1] = 
            &question[correct+1][0..(question[correct+1].len()-1)];

        out.push(
            Question{
                text: question[0].to_owned(),
                answers: question[1..5]
                    .iter().map(|x| String::from(*x)).collect(),
                correct 
            }
        );

    }

}
