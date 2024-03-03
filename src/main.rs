struct QOption {
    options : Vec<String>,
    correct : usize
}

impl QOption{
    fn new(options : Vec<String>, correct : usize) -> Self {
        Self{
            options,
            correct
        }
    }

    fn is_correct(&self, idx : usize) -> bool {
        return idx == self.correct;
    }
}

struct Quiz{
    questions : Vec<String>,
    options : Vec<QOption>,
    correct : usize,
    wrong : usize,
    done : usize,
}

impl Quiz{
    fn ask_question(&self, idx : usize) -> bool {
        println!("Q : {}", self.questions[idx]);
        for (i, option) in self.options[idx].options.iter().enumerate() {
            println!("{i}) {option}");
        }
        println!("Enter your choice");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let index = choice.trim().parse::<usize>().expect("expected a usize");
        if self.options[idx].is_correct(index) {
            return true
        }
        false
    }

    fn start(&mut self) {
        self.correct = 0;
        self.wrong = 0;
        while self.done < self.questions.len() {
            let correct = self.ask_question(self.done);
            if correct {
                self.correct += 1
            }
            self.done += 1
        }
        self.end()
    }
    
    fn end(&self){
        println!("Your got {}/{} correct!", self.correct, self.questions.len());
    }
}

fn main(){
    let questions = [String::from("Who's the father of Ezio Auditore?"), String::from("Where was Ezio born?")];
    let qoptions = vec![String::from("Guiovanni"), String::from("Francesco"), String::from("Vieri"), String::from("Federico")];
    let qoptions2 = vec![String::from("Tuscany"), String::from("Monteriggioni"), String::from("Florence"), String::from("Venezia (Venice)")];
    let option = QOption::new(qoptions, 0);
    let option2 = QOption::new(qoptions2, 2);
    let options = vec![option, option2];
    let mut quiz = Quiz { questions:  questions.to_vec(), options, correct : 0, wrong : 0, done : 0 };
    quiz.start();
}
